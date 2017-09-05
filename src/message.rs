use std::convert::TryFrom;
use std::io::Read;
use errors::*;
use object::{Str,DecodableObject};

#[derive(Debug,PartialEq,Eq,Hash)]
pub(crate) enum Id {
    BufferOpened,
    BufferTypeChanged,
    BufferMoved,
    BufferMerged,
    BufferUnmerged,
    BufferHidden,
    BufferUnhidden,
    BufferRenamed,
    BufferTitleChanged,
    BufferLocalVarAdded,
    BufferLocalVarChanged,
    BufferLocalVarRemoved,
    BuffferClosing,
    BufferCleared,
    BufferLineAdded,
    Nicklist,
    NicklistDiff,
    Pong,
    Upgrade,
    UpgradeEnded,
    Other(String),
}

impl<'a, R: Read + ?Sized> TryFrom<&'a mut R> for Id {
    type Error = Error;

    fn try_from(reader: &mut R) -> Result<Self> {
        println!(">>> Reading id");
        let id = Str::decode_bare(reader)?;
        println!("Id: {:?}", id);

        Ok(match id.as_str() {
            "_buffer_opened" => Id::BufferOpened,
            "_buffer_type_changed" => Id::BufferTypeChanged,
            "_buffer_moved" => Id::BufferMoved,
            "_buffer_merged" => Id::BufferMerged,
            "_buffer_unmerged" => Id::BufferUnmerged,
            "_buffer_hidden" => Id::BufferHidden,
            "_buffer_unhidden" => Id::BufferUnhidden,
            "_buffer_renamed" => Id::BufferRenamed,
            "_buffer_title_changed" => Id::BufferTitleChanged,
            "_buffer_localvar_added" => Id::BufferLocalVarAdded,
            "_buffer_localvar_changed" => Id::BufferLocalVarChanged,
            "_buffer_localvar_removed" => Id::BufferLocalVarRemoved,
            "_buffer_closing" => Id::BuffferClosing,
            "_buffer_cleared" => Id::BufferCleared,
            "_buffer_line_added" => Id::BufferLineAdded,
            "_nicklist" => Id::Nicklist,
            "_nicklist_diff" => Id::NicklistDiff,
            "_pong" => Id::Pong,
            "_upgrade" => Id::Upgrade,
            "_upgrade_ended" => Id::UpgradeEnded,
            _ => Id::Other(id.into()),
        })
    }
}

#[derive(Debug)]
pub enum Message {
    Pong(Pong)
}

#[derive(Debug)]
pub struct Pong(pub Str);

impl<'a, R: Read + ?Sized> TryFrom<&'a mut R> for Pong {
    type Error = Error;

    fn try_from(reader: &mut R) -> Result<Self> {
        println!(">>> Decoding [Pong]");
        let msg = Str::decode(reader)?;

        Ok(Pong(msg))
    }
}

impl From<Pong> for Message {
    fn from(m: Pong) -> Self {
        Message::Pong(m)
    }
}
