use std::io::Read;
use super::{Tag,DecodableObject,Int};
use errors::*;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Buffer(Option<Vec<u8>>);

impl DecodableObject for Buffer {
    const TAG: Tag = b"buf";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let len = Int::decode_bare(r)?;

        if len == -1 {
            return Ok(Buffer(None));
        }

        let mut buffer = Vec::with_capacity(len as usize);

        r.take(len as u64).read_to_end(&mut buffer)?;

        Ok(Buffer(Some(buffer)))
    }
}

impl From<Buffer> for Vec<u8> {
    fn from(s: Buffer) -> Self {
        s.0.unwrap_or_else(Vec::new)
    }
}

impl From<Buffer> for Option<Vec<u8>> {
    fn from(s: Buffer) -> Self {
        s.0
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(s: Vec<u8>) -> Self {
        Buffer(Some(s))
    }
}

impl From<Option<Vec<u8>>> for Buffer {
    fn from(s: Option<Vec<u8>>) -> Self {
        Buffer(s)
    }
}

impl<'a> From<&'a [u8]> for Buffer {
    fn from(s: &[u8]) -> Self {
        Buffer(Some(Vec::from(s)))
    }
}
