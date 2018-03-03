extern crate core;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::btree_map::BTreeMap;
use ser::{Serializer, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
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

impl Value {
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        if let Value::Array(ref mut v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        if let Value::Bytes(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_bytes_mut(&mut self) -> Option<&mut Vec<u8>> {
        if let Value::Bytes(ref mut v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        if let Value::String(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        if let Value::String(ref mut v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<u32> {
        match *self {
            Value::Int(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<Value, Value>> {
        if let Value::Map(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_map_mut(&mut self) -> Option<&BTreeMap<Value, Value>> {
        if let Value::Map(ref mut v) = *self {
            Some(v)
        } else {
            None
        }
    }
}
