use std::io::Read;
use byteorder::ReadBytesExt;
use super::{Tag,DecodableObject};
use errors::*;

pub type Long = i64;

impl DecodableObject for Long {
    const TAG: Tag = b"lon";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let len = r.read_u8()?;
        let mut buf = String::with_capacity(len as usize);

        r.take(len as u64).read_to_string(&mut buf)?;

        let n = buf.parse::<i64>().chain_err(|| ErrorKind::Decoding)?;

        Ok(n)
    }
}
