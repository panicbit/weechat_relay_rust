use std::io::Read;
use std::fmt;
use byteorder::ReadBytesExt;
use super::{Tag,DecodableObject,Buffer};
use errors::*;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Time(String);

impl DecodableObject for Time {
    const TAG: Tag = b"tim";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let len = r.read_u8()?;
        let mut buf = String::with_capacity(len as usize);

        r.take(len as u64).read_to_string(&mut buf)?;

        Ok(Time(buf))
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<u64> for Time {
    fn from(time: u64) -> Time {
        Time(time.to_string())
    }
}
