use std::io::Read;
use byteorder::ReadBytesExt;
use super::{Tag,DecodableObject};
use errors::*;

pub type Char = i8;

impl DecodableObject for Char {
    const TAG: Tag = b"chr";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let n = r.read_i8()?;
        Ok(n)
    }
}
