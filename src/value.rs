extern crate core;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::btree_map::BTreeMap;
use ser::{Serializer, Serialize};

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(u32),
    NegativeInt(u32),
    String(String),
    Bytes(Vec<u8>),
    Text(String),
    Array(Vec<Value>),
    Map(BTreeMap<Value, Value>),
}

impl Serialize for Value {
    fn serialize<S>(&mut self, mut serializer: S) where S: Serializer {
        match *self {
            Value::Int(n) => serializer.serialize_unsigned(n, 0),
            Value::Bytes(ref bytes) => serializer.serialize_bytes(bytes.to_vec()),
            Value::Array(ref mut array) => serializer.serialize_seq(array),
            _ => unreachable!(),
        }
    }
}
