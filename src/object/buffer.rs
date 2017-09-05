use std::io::Read;
use super::{Tag,DecodableObject,Int};
use errors::*;

pub type Buffer = Option<Vec<u8>>;

impl DecodableObject for Buffer {
    const TAG: Tag = b"buf";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let len = Int::decode_bare(r)?;

        if len == -1 {
            return Ok(None);
        }

        let mut buffer = Vec::with_capacity(len as usize);

        r.take(len as u64).read_to_end(&mut buffer)?;

        Ok(Some(buffer))
    }
}
