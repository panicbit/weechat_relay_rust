use std::io::Read;
use byteorder::ReadBytesExt;
use super::{Tag,DecodableObject,Buffer};
use errors::*;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Time(pub(super) u64);

impl DecodableObject for Time {
    const TAG: Tag = b"tim";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let len = r.read_u8()?;
        let mut buf = String::with_capacity(len as usize);

        r.take(len as u64).read_to_string(&mut buf)?;

        let n = buf.parse::<u64>().chain_err(|| ErrorKind::Decoding)?;

        Ok(Time(n))
    }
}
