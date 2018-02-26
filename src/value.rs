extern crate core;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::btree_map::BTreeMap;
use ser::{Serializer, Serialize};

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(u64),
    NegativeInt(u64),
    Bytes(Vec<u8>),
    Text(String),
    Array(Vec<Value>),
    Map(BTreeMap<Value, Value>),
}

impl Serialize for Value {
    fn serialize<S>(&mut self, serializer: S) where S: Serializer {
        match *self {
            Value::Int(n) => serializer.serialize_unsigned(n),
            _ => unreachable!(),
        }
        println!("Serializing {:?}", *self);
    }
}
