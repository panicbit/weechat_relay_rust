use std::io::Read;
use byteorder::{ReadBytesExt,BigEndian as BE};
use super::{Tag,DecodableObject};
use errors::*;

pub type Int = i32;

impl DecodableObject for Int {
    const TAG: Tag = b"int";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let n = r.read_i32::<BE>()?;
        Ok(n)
    }
}
