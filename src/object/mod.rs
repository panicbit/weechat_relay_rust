use std::io::Read;
use byteorder::{ReadBytesExt, BigEndian as BE};
use errors::*;
use std::collections::HashMap;
use std::hash::{Hash,Hasher};
use std::ops::{Deref,DerefMut};
use std::fmt;

mod str;
mod buffer;
mod pointer;
mod time;
mod hash_table;
mod info;
mod info_list;
mod array;
mod char;
mod int;
mod long;

pub use self::str::Str;
pub use self::buffer::Buffer;
pub use self::pointer::Pointer;
pub use self::time::Time;
pub use self::hash_table::HashTable;
pub use self::info::Info;
pub use self::info_list::InfoList;
pub use self::array::Array;
pub use self::char::Char;
pub use self::int::Int;
pub use self::long::Long;

#[derive(Debug,PartialEq,Eq,Hash)]
pub enum Object {
    Char(Char),
    Int(Int),
    Long(Long),
    Str(Str),
    Buffer(Buffer),
    Pointer(Pointer),
    Time(Time),
    HashTable(HashTable),
    Info(Info),
    InfoList(InfoList),
    Array(Array),
}

impl Object {
    fn decode<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let tag = read_tag(r).chain_err(|| "reading tag")?;
        let decode = Self::bare_decoder_from_tag(tag)?;
        decode(r).chain_err(|| "decoding object")
    }

    fn bare_decoder_from_tag<R: Read + ?Sized>(tag: [u8; 3]) -> Result<fn(&mut R) -> Result<Self>> {
        Ok(match &tag {
            Char     ::TAG => |r| Char     ::decode_bare(r).map(Object::Char     ).chain_err(|| "Decoding Char"     ),
            Int      ::TAG => |r| Int      ::decode_bare(r).map(Object::Int      ).chain_err(|| "Decoding Int"      ),
            Long     ::TAG => |r| Long     ::decode_bare(r).map(Object::Long     ).chain_err(|| "Decoding Long"     ),
            Str      ::TAG => |r| Str      ::decode_bare(r).map(Object::Str      ).chain_err(|| "Decoding Str"      ),
            Buffer   ::TAG => |r| Buffer   ::decode_bare(r).map(Object::Buffer   ).chain_err(|| "Decoding Buffer"   ),
            Pointer  ::TAG => |r| Pointer  ::decode_bare(r).map(Object::Pointer  ).chain_err(|| "Decoding Pointer"  ),
            Time     ::TAG => |r| Time     ::decode_bare(r).map(Object::Time     ).chain_err(|| "Decoding Time"     ),
            HashTable::TAG => |r| HashTable::decode_bare(r).map(Object::HashTable).chain_err(|| "Decoding HashTable"),
            Info     ::TAG => |r| Info     ::decode_bare(r).map(Object::Info     ).chain_err(|| "Decoding Info"     ),
            InfoList ::TAG => |r| InfoList ::decode_bare(r).map(Object::InfoList ).chain_err(|| "Decoding InfoList" ),
            Array    ::TAG => |r| Array    ::decode_bare(r).map(Object::Array    ).chain_err(|| "Decoding Array"    ),
            _ => bail!(ErrorKind::UnknownTag(tag)),
        })
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Char(ch) => ch.fmt(f),
            Object::Int(n) => n.fmt(f),
            Object::Long(n) => n.fmt(f),
            Object::Str(ref s) => s.as_str().fmt(f),
            Object::Buffer(..) => write!(f, "<buffer>"),
            Object::Pointer(ref ptr) => ptr.fmt(f),
            Object::Time(Time(time)) => time.fmt(f),
            Object::HashTable(..) => write!(f, "<hash_table>"),
            Object::Info(Info { ref name, ref value }) => write!(f, "({} => {})", name.as_str(), value.as_str()),
            Object::InfoList(..) => write!(f, "<info_list>"),
            Object::Array(..) => write!(f, "<array>"),
        }
    }
}

pub type Tag = &'static [u8; 3];

fn read_tag<R: Read + ?Sized>(r: &mut R) -> Result<[u8; 3]> {
    let mut tag = [0; 3];
    r.read_exact(&mut tag[..])?;
    Ok(tag)
}

pub(crate) trait DecodableObject {
    const TAG: Tag;

    fn decode<R: Read + ?Sized>(r: &mut R) -> Result<Self> where Self: Sized {
        let tag = read_tag(r)?;

        ensure!(&tag == Self::TAG, ErrorKind::UnexpectedType);

        Self::decode_bare(r)
    }

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> where Self: Sized;
}
