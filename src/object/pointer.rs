use std::io::Read;
use std::fmt;
use byteorder::ReadBytesExt;
use super::{Tag,DecodableObject};
use errors::*;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Pointer(String);

impl DecodableObject for Pointer {
    const TAG: Tag = b"ptr";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let len = r.read_u8()?;
        let mut pointer = String::with_capacity(len as usize);

        r.take(len as u64).read_to_string(&mut pointer)?;

        // TODO: verify that `pointer` is hex encoded

        Ok(Pointer(pointer))
    }
}

impl fmt::Display for Pointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", self.0)
    }
}

impl From<usize> for Pointer {
    fn from(ptr: usize) -> Self {
        Pointer(format!("{:x}", ptr))
    }
}
