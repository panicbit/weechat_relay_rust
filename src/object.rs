use std::io::Read;
use byteorder::{ReadBytesExt, BigEndian as BE};
use errors::*;
use std::collections::HashMap;
use std::hash::{Hash,Hasher};
use std::ops::{Deref,DerefMut};
use std::fmt;

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
            Object::Pointer(Pointer(ref str)) => write!(f, "0x{}", str),
            Object::Time(Time(time)) => time.fmt(f),
            Object::HashTable(..) => write!(f, "<hash_table>"),
            Object::Info(Info { ref name, ref value }) => write!(f, "({} => {})", name.as_str(), value.as_str()),
            Object::InfoList(..) => write!(f, "<info_list>"),
            Object::Array(..) => write!(f, "<array>"),
        }
    }
}

pub type Tag = &'static [u8; 3];
pub type Char = i8;
pub type Int = i32;
pub type Long = i64;

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

pub type Buffer = Option<Vec<u8>>;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Pointer(String);

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Time(u64);

#[derive(Debug,PartialEq,Eq)]
pub struct HashTable(pub HashMap<Object,Object>);

impl Hash for HashTable {
    fn hash<H>(&self, _state: &mut H) where
        H: Hasher
    {
        panic!("Cannot hash HashTable");
    }
}

impl Deref for HashTable {
    type Target = HashMap<Object,Object>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HashTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Info {
    name: Str,
    value: Str,
}

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct InfoList {
    pub name: Str,
    pub items: Vec<HashTable>,
}

pub type Array = Vec<Object>;

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

impl DecodableObject for Char {
    const TAG: Tag = b"chr";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let n = r.read_i8()?;
        Ok(n)
    }
}

impl DecodableObject for Int {
    const TAG: Tag = b"int";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let n = r.read_i32::<BE>()?;
        Ok(n)
    }
}

impl DecodableObject for Long {
    const TAG: Tag = b"lon";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let len = r.read_u8()?;
        let mut buf = String::with_capacity(len as usize);

        r.take(len as u64).read_to_string(&mut buf)?;

        let n = buf.parse::<i64>().chain_err(|| ErrorKind::Decoding)?;

        Ok(n)
    }
}

impl DecodableObject for Str {
    const TAG: Tag = b"str";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let buffer = match Buffer::decode_bare(r)? {
            None => return Ok(Str::from(None)),
            Some(buffer) => buffer,
        };

        
        let string = String::from_utf8(buffer).chain_err(|| "err, not UTF8")?;

        Ok(Str::from(string))
    }
}

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

impl DecodableObject for Pointer {
    const TAG: Tag = b"ptr";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let len = r.read_u8()?;
        let mut pointer = String::with_capacity(len as usize);

        r.take(len as u64).read_to_string(&mut pointer)?;

        Ok(Pointer(pointer))
    }
}

impl DecodableObject for Time {
    const TAG: Tag = b"tim";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let len = r.read_u8()?;
        let mut buf = String::with_capacity(len as usize);

        r.take(len as u64).read_to_string(&mut buf)?;

        let n = buf.parse::<u64>().chain_err(|| ErrorKind::Decoding)?;

        Ok(Time(n))
    }
}

impl DecodableObject for HashTable {
    const TAG: Tag = b"htb";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let key_tag = read_tag(r)?;
        let value_tag = read_tag(r)?;
        let decode_key = Object::bare_decoder_from_tag(key_tag)?;
        let decode_value = Object::bare_decoder_from_tag(value_tag)?;
        let len = r.read_u32::<BE>()?;
        let mut hm = HashMap::with_capacity(len as usize);

        for _ in 0..len {
            let key = decode_key(r)?;
            let value = decode_value(r)?;
            hm.insert(key, value);
        }

        Ok(HashTable(hm))
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

impl DecodableObject for InfoList {
    const TAG: Tag = b"inl";

    fn decode_bare<R: Read + ?Sized>(r: &mut R) -> Result<Self> {
        let name = Str::decode_bare(r).chain_err(|| "decoding name")?;
        println!(">>> name: {}", name.as_str());
        let len = r.read_u32::<BE>().chain_err(|| "decoding len")?;
        println!(">>> num_entries: {}", len);
        let mut items = Vec::with_capacity(len as usize);


        for _ in 0..len {
            let len = r.read_u32::<BE>().chain_err(|| "decoding num items")?;
            let mut item = HashMap::with_capacity(len as usize);

            for _ in 0..len {
                let name = Str::decode_bare(r).map(Object::Str).chain_err(|| "decoding item name")?;
                let value = Object::decode(r).chain_err(|| format!("decoding item value of '{}'", name))?;

                item.insert(name, value);
            }

            items.push(HashTable(item));
        }

        Ok(InfoList { name, items })
    }
}

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
