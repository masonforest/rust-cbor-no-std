use alloc::btree_map::BTreeMap;
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
        let header_byte = self.reader.read_byte();
        let major_type = bytes::major_type(header_byte);
        let additional_type = self.read_additional_type(bytes::additional_type(header_byte));

        match major_type {
            0b000 => self.deserialize_int(additional_type),
            0b010 => self.deserialize_bytes(additional_type),
            0b011 => self.deserialize_string(additional_type),
            0b100 => self.deserialize_array(additional_type),
            0b101 => self.deserialize_map(additional_type),
            0b111 => self.deserialize_simple(additional_type),
            _ => unreachable!(),
        }
    }

    fn read_additional_type(&mut self, additional_type: u8) -> usize {
        match additional_type {
            0b00000...0b10111 => additional_type as usize,
            0b11000 => self.reader.read_byte() as usize,
            _ => unreachable!(),
        }
    }

    fn deserialize_int(&self, value: usize) -> Value{
        Value::Int(value as u32)
    }

    fn deserialize_bytes(&mut self, len: usize) -> Value{
        Value::Bytes(self.reader.read_n_bytes(len as usize))
    }

    fn deserialize_string(&mut self, len: usize) -> Value {
        bytes::to_string(&self.reader.read_n_bytes(len as usize))
    }

    fn deserialize_array(&mut self, len: usize) -> Value {
        let values = (0..len).map(|_| self.parse_value()).collect();
        Value::Array(values)
    }

    fn deserialize_map(&mut self, len: usize) -> Value {
        let mut map = BTreeMap::new();

        for _ in 0..len {
            let key = self.parse_value().as_string().unwrap().clone();
            let value = self.parse_value();

            map.insert(key, value);
        }

        Value::Map(map)
    }

    fn deserialize_simple(&mut self, value: usize) -> Value {
        match value {
            22 => Value::Null,
            _ => unreachable!(),
        }
    }
}

#[test]
fn deserialize_map() {
    let mut test_map = BTreeMap::new();
    test_map.insert("key1".into(), Value::String("value1".into()));
    test_map.insert("key2".into(), Value::String("value2".into()));
    let expected: Value = Value::Map(test_map);
    assert_eq!(expected, from_bytes(vec![0xa2, 0x64, 0x6b, 0x65, 0x79, 0x31, 0x66, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x31, 0x64, 0x6b, 0x65, 0x79, 0x32, 0x66, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x32]));
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

#[test]
fn deserialize_null() {
    let expected: Value = Value::Null;
    assert_eq!(expected, from_bytes(vec![246]));
}
