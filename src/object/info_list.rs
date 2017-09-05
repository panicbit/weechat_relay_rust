use std::io::Read;
use std::collections::HashMap;
use byteorder::{ReadBytesExt,BigEndian as BE};
use super::{Object,HashTable,Str,Tag,DecodableObject};
use errors::*;

#[derive(Debug,PartialEq,Eq,Hash)]
pub struct InfoList {
    pub name: Str,
    pub items: Vec<HashTable>,
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
