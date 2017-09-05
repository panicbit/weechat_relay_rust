use std::io::Read;
use super::{Tag,DecodableObject,Str};
use errors::*;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Info {
    pub(super) name: Str,
    pub(super) value: Str,
}

impl DecodableObject for Info {
    const TAG: Tag = b"inf";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        Ok(Info {
            name: Str::decode_bare(r)?,
            value: Str::decode_bare(r)?,
        })
    }
}
