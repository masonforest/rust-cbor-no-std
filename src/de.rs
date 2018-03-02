// use alloc::string::String;
use alloc::btree_map::BTreeMap;
#[cfg(all(feature = "no_std", not(test)))]
use core::str;
#[cfg(any(not(feature = "no_std"), test))]
use std::str;
use alloc::vec::Vec;
use value::Value;
use bytes::bytes;
use io::{Reader, VecReader};

pub struct Deserializer<R: Reader>  {
    pub reader: R,
}

pub fn from_bytes(bytes: Vec<u8>) -> Value
{
    let vec_reader = VecReader{input: bytes};
    let mut d = Deserializer::from_vec_reader(vec_reader);
    d.parse_value()
}

pub enum TestValue {
    Int(u64),
}

impl<R> Deserializer<R> where R: Reader {
    fn from_vec_reader(bytes: R) -> Deserializer<R> {
        Deserializer{reader: bytes}
    }

    fn parse_value(&mut self) -> Value
    {
        let byte = self.reader.read_byte();
        let major_type = bytes::major_type(byte);
        let additional_type = self.read_additional_type(bytes::additional_type(byte));

        match major_type {
            0b000 => self.deserialize_int(additional_type),
            0b010 => self.deserialize_bytes(additional_type),
            0b011 => self.deserialize_string(additional_type),
            0b100 => self.deserialize_array(additional_type),
            0b101 => self.deserialize_map(additional_type),
            _ => unreachable!(),
        }
    }

    fn read_additional_type(&mut self, additional_type: u8) -> Vec<u8> {
        match additional_type {
            0b00000...0b10111 => vec![additional_type],
            0b11000 => vec![self.reader.read_byte()],
            _ => unreachable!(),
        }
    }

    fn deserialize_int(&self, bytes: Vec<u8>) -> Value{
        Value::Int(bytes[0] as u32)
    }

    fn deserialize_bytes(&mut self, len: Vec<u8>) -> Value{
        Value::Bytes(self.reader.read_n_bytes(len[0] as usize))
    }

    fn deserialize_string(&mut self, len: Vec<u8>) -> Value {
        bytes::to_string(&self.reader.read_n_bytes(len[0] as usize))
    }

    fn deserialize_array(&mut self, len: Vec<u8>) -> Value {
        let values = (0..len[0]).map(|_| self.parse_value()).collect();
        Value::Array(values)
    }

    fn deserialize_map(&mut self, len: Vec<u8>) -> Value {
        let bytes = self.reader.read_n_bytes(len[0] as usize);
        let len = self.reader.read_byte() as usize - 0x60;
        let key = bytes::to_string(&self.reader.read_n_bytes(len));
        let value = bytes::to_string(&self.reader.read_n_bytes(len));

        let mut test_map = BTreeMap::new();
        test_map.insert(key, value);
        Value::Map(test_map)
    }
}

#[test]
fn deserialize_map() {
    let mut test_map = BTreeMap::new();
    test_map.insert(Value::String("a".into()), Value::String("b".into()));
    let expected: Value = Value::Map(test_map);
    assert_eq!(expected, from_bytes(vec![0xa1, 0x61, 0x61, 0x61, 0x62]));
}

#[test]
fn deserialize_string() {
    let expected: Value = Value::String("test".into());
    assert_eq!(expected, from_bytes(vec![0x64, 0x74, 0x65, 0x73, 0x74]));
}

#[test]
fn deserialize_array() {
    let expected: Value = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
    assert_eq!(expected, from_bytes(vec![0x83, 0x01, 0x02, 0x03]));
}

#[test]
fn deserialize_bytes() {
    let expected: Value = Value::Bytes(vec![1, 2, 3]);
    assert_eq!(expected, from_bytes(vec![0x43, 0x01, 0x02, 0x03]));
}

#[test]
fn deserialize_u8() {
    let expected: Value = Value::Int(1);
    assert_eq!(expected, from_bytes(vec![0x01]));
}

#[test]
fn deserialize_u8_2() {
    let expected: Value = Value::Int(42);
    assert_eq!(expected, from_bytes(vec![24, 42]));
}
