use std::io::Read;
use byteorder::{ReadBytesExt,BigEndian as BE};
use super::{Object,Tag,DecodableObject,read_tag};
use errors::*;

pub type Array = Vec<Object>;

impl DecodableObject for Array {
    const TAG: Tag = b"arr";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let elem_tag = read_tag(r)?;
        let decode_elem = Object::bare_decoder_from_tag(elem_tag)?;
        let len = r.read_u32::<BE>()?;
        let mut array = Vec::with_capacity(len as usize);

        for _ in 0..len {
            let object = decode_elem(r).chain_err(|| ErrorKind::Decoding)?;
            array.push(object);
        }

        Ok(array)
    }
}
