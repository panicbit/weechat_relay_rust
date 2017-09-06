use std::io::Read;
use std::ops::Deref;
use super::{Tag,DecodableObject,Buffer};
use errors::*;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Str(Option<String>);

impl Str {
    pub fn as_ref(&self) -> Option<&str> {
        self.0.as_ref().map(String::as_str)
    }

    pub fn as_str(&self) -> &str {
        self.as_ref().unwrap_or("")
    }
}

impl DecodableObject for Str {
    const TAG: Tag = b"str";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let buffer = match Buffer::decode_bare(r).map(Option::from)? {
            None => return Ok(Str::from(None)),
            Some(buffer) => buffer,
        };

        let string = String::from_utf8(buffer).chain_err(|| "err, not UTF8")?;

        Ok(Str::from(string))
    }
}

impl Deref for Str {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl From<Str> for String {
    fn from(s: Str) -> Self {
        s.0.unwrap_or_else(String::new)
    }
}

impl From<Str> for Option<String> {
    fn from(s: Str) -> Self {
        s.0
    }
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str(Some(s))
    }
}

impl From<Option<String>> for Str {
    fn from(s: Option<String>) -> Self {
        Str(s)
    }
}

impl<'a> From<&'a str> for Str {
    fn from(s: &str) -> Self {
        Str(Some(String::from(s)))
    }
}
