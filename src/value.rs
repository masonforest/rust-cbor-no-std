extern crate core;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;
use ser::{Serializer, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum Value {
    Null,
    Int(u64),
    NegativeInt(u64),
    String(String),
    Bytes(Vec<u8>),
    Text(String),
    Array(Vec<Value>),
    Map(BTreeMap<String, Value>),
}

impl Serialize for Value {
    fn serialize<S>(&mut self, serializer: &mut S) where S: Serializer {
        match *self {
            Value::Int(n) => serializer.serialize_unsigned(n as u64, 0),
            Value::Bytes(ref bytes) => serializer.serialize_bytes(bytes.to_vec()),
            Value::String(ref string) => serializer.serialize_string(string),
            Value::Array(ref mut array) => {
                serializer.serialize_seq(array.len());

                for value in array {
                    value.serialize(serializer);
                }
            },
            Value::Map(ref mut map) => {
                serializer.serialize_map(map.len());
                for(key, value) in map {
                    serializer.serialize_string(key);
                    value.serialize(serializer);
                }
            },
            Value::Null => serializer.serialize_simple(22),
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

    pub fn as_int(&self) -> Option<u64> {
        match *self {
            Value::Int(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<String, Value>> {
        if let Value::Map(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_map_mut(&mut self) -> Option<&BTreeMap<String, Value>> {
        if let Value::Map(ref mut v) = *self {
            Some(v)
        } else {
            None
        }
    }
}

impl From<()> for Value {
    fn from(_value: ()) -> Value {
        Value::Null
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Value {
        Value::Int(value as u64)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Value {
        Value::Bytes(value)
    }
}
