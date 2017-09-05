use futures::prelude::*;
use tokio_io::io;
use tokio_io::{AsyncRead,AsyncWrite};
use std::io::{Read,BufRead};
use std::convert::TryFrom;
use std::mem::size_of;
use byteorder::{ReadBytesExt,BigEndian};
use errors::*;
use libflate::zlib;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Compression {
    Off,
    Zlib,
}

impl TryFrom<u8> for Compression {
    type Error = Error;

    fn try_from(byte: u8) -> Result<Self> {
        Ok(match byte {
            0 => Compression::Off,
            1 => Compression::Zlib,
            code => Err(ErrorKind::UnknownCompression(code))?,
        })
    }
}

#[async]
pub fn send_init<W: AsyncWrite + 'static>(writer: W, pass: String, compression: Compression) -> Result<W> {
    let pass = pass.replace(",", r"\,");

    let init = format!("init compression=off,password={}\n", pass);

    let (writer, _) = await!(io::write_all(writer, init))?;

    Ok(writer)
}

#[async]
pub fn read_message<R: AsyncRead + 'static>(reader: R) -> Result<(R, Vec<u8>)> {
    // Get message length
    let (reader, len        ) = await!(read_u32(reader))?;
    ensure!(len >= size_of::<u32>() as u32, ErrorKind::InvalidMessageLength);
    let len = len - 4;

    // Get message compression
    ensure!(len >= 1 as u32, ErrorKind::InvalidMessageLength);
    let (reader, compression) = await!(read_compression(reader))?;
    let len = len - 1;

    let (reader, data) = await!(decompress(reader, len as usize, compression))?;

    println!("Received (head):");
    for line in ::hexdump::hexdump_iter(&data).take(10) {
        println!("{}", line);
    }

    Ok((reader, data))
}

#[async]
fn read_u32<R: AsyncRead + 'static>(reader: R) -> Result<(R, u32)> {
    let (reader, buf) = await!(io::read_exact(reader, [0; 4]))?;
    let n = buf.as_ref().read_u32::<BigEndian>()?;

    Ok((reader, n))
}

#[async]
fn read_compression<R: AsyncRead + 'static>(reader: R) -> Result<(R, Compression)> {
    let (reader, buf) = await!(io::read_exact(reader, [0; 1]))?;

    let compression = Compression::try_from(buf[0])?;

    Ok((reader, compression))
}

#[async]
fn decompress<R: AsyncRead + 'static>(reader: R, len: usize, compression: Compression) -> Result<(R, Vec<u8>)> {
    let raw = vec![0; len];
    let (reader, raw) = await!(io::read_exact(reader, raw))?;

    let decoded = match compression {
        Compression::Off => raw,
        Compression::Zlib => {
            let mut data = Vec::new();
            zlib::Decoder::new(raw.as_slice())?.read_to_end(&mut data)?;
            data
        },
    };

    Ok((reader, decoded))
}
