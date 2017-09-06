use std::io::Read;
use super::{Tag,DecodableObject,Str};
use errors::*;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Info {
    name: Str,
    value: Str,
}

impl Info {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
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

impl<S1,S2> From<(S1,S2)> for Info where
    S1: Into<String>,
    S2: Into<String>,
{
    fn from((name, value): (S1,S2)) -> Self {
        Info {
            name: name.into().into(),
            value: value.into().into(),
        }
    }
}
