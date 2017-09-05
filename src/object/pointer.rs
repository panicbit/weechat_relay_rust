use std::io::Read;
use byteorder::ReadBytesExt;
use super::{Tag,DecodableObject};
use errors::*;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Pointer(pub(super) String);

impl DecodableObject for Pointer {
    const TAG: Tag = b"ptr";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let len = r.read_u8()?;
        let mut pointer = String::with_capacity(len as usize);

        r.take(len as u64).read_to_string(&mut pointer)?;

        Ok(Pointer(pointer))
    }
}
