use std::io::Read;
use std::collections::HashMap;
use std::ops::{Deref,DerefMut};
use byteorder::{ReadBytesExt,BigEndian as BE};
use std::hash::{Hash,Hasher};
use super::{Object,Tag,DecodableObject,read_tag};
use errors::*;

#[derive(Debug,PartialEq,Eq)]
pub struct HashTable(HashMap<Object,Object>);

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

impl<K,V> From<HashMap<K,V>> for HashTable where
    K: Into<Object> + Eq + Hash,
    V: Into<Object>,
{
    fn from(hm: HashMap<K,V>) -> Self {
        let hm = hm.into_iter()
            .map(|(k,v)| (k.into(), v.into()))
            .collect::<HashMap<_,_>>();

        HashTable(hm)
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
