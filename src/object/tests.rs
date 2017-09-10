use std::fmt::Debug;
use super::*;

fn test<T,E>(mut data: &[u8], expected: E) where
    T: DecodableObject + Debug + PartialEq,
    E: Into<T>,
{
    let actual = T::decode_bare(&mut data).unwrap();
    assert_eq!(actual, expected.into());
}

#[test]
fn char() {
    assert_eq!(Char::TAG, b"chr");
    test::<Char,_>(b"\x41", b'A' as Char);
}

#[test]
fn int() {
    assert_eq!(Int::TAG, b"int");
    test::<Int,_>(b"\x00\x01\xE2\x40", 123456);
    test::<Int,_>(b"\xFF\xFE\x1D\xC0", -123456);
}

#[test]
fn long() {
    assert_eq!(Long::TAG, b"lon");
    test::<Long,_>(b"\x0A1234567890", 1234567890);
    test::<Long,_>(b"\x0B-1234567890", -1234567890);
}

#[test]
fn string() {
    assert_eq!(Str::TAG, b"str");
    test::<Str,_>(b"\0\0\0\x05hello", "hello");
    test::<Str,_>(b"\0\0\0\0", "");
    test::<Str,_>(b"\xff\xff\xff\xff", None);
}

#[test]
fn buffer() {
    assert_eq!(Buffer::TAG, b"buf");
    test::<Buffer,_>(b"\0\0\0\x05hello", &b"hello"[..]);
    test::<Buffer,_>(b"\0\0\0\0", &[][..]);
    test::<Buffer,_>(b"\xff\xff\xff\xff", None);
}

#[test]
fn pointer() {
    assert_eq!(Pointer::TAG, b"ptr");
    test::<Pointer,_>(b"\x091a2b3c4d5", 0x1a2b3c4d5);
    test::<Pointer,_>(b"\x010", 0x0);
}

#[test]
fn time() {
    assert_eq!(Time::TAG, b"tim");
    test::<Time,_>(b"\x0A1321993456", 1321993456);
}

#[test]
fn hash_table() {
    assert_eq!(HashTable::TAG, b"htb");
    test::<HashTable,_>(
        b"strstr\0\0\0\x02\0\0\0\x04key1\0\0\0\x03abc\0\0\0\x04key2\0\0\0\x03def",
        hashmap! {
            Object::str("key1") => Object::str("abc"),
            Object::str("key2") => Object::str("def"),
        }
    );
}

#[test]
fn hdata() {
    unimplemented!("hdata");
}

#[test]
fn info() {
    assert_eq!(Info::TAG, b"inf");
    test::<Info,_>(b"\0\0\0\x04name\0\0\0\x05value", ("name", "value"));
}

#[test]
fn info_list() {
    assert_eq!(InfoList::TAG, b"inl");
    unimplemented!();
}

#[test]
fn array() {
    assert_eq!(Array::TAG, b"arr");
    test::<Array,_>(b"str\0\0\0\x02\0\0\0\x03abc\0\0\0\x02de", vec![Object::str("abc"), Object::str("de")]);
    test::<Array,_>(b"int\0\0\0\x03\0\0\0\x7B\0\0\x01\xC8\0\0\x03\x15", vec![Object::int(123), Object::int(456), Object::int(789)]);
}
