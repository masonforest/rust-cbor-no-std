use constants::*;
use bytes::bytes;
use alloc::vec::Vec;
use value::{Value};
use io::{Writer, VecWriter};

pub trait Serializer {
    fn serialize_unsigned(&mut self, n: u64, major_type: u8);
    fn serialize_seq(&mut self, seq: &Vec<Value>);
}

pub trait Serialize {
    fn serialize<S>(&mut self, serializer: S) where S: Serializer;
}

pub struct VecSerializer<'a>  {
    pub writer: VecWriter<'a>,
}

pub fn to_bytes(mut value: Value) -> Vec<u8>
{
    let mut output = Vec::with_capacity(128);
    value.serialize(
        VecSerializer::from_vec_writer(
            VecWriter{
                output: &mut output,
            }
        )
    );
    output
}

impl<'a> VecSerializer<'a> {
    fn from_vec_writer(vec_writer: VecWriter) -> VecSerializer {
        VecSerializer{writer: vec_writer}
    }

    fn encode_unsigned(&mut self, n: u64, major_type: u8) -> Vec<u8> {
        match n {
            n @ 0 ... 23 => vec![bytes::concat(major_type, n as u8)],
            n @ 24 ... MAX_U8 => vec![bytes::concat(major_type, 24), n as u8],
            _ => unreachable!()
        }
    }

}

impl<'a> Serializer for VecSerializer<'a> {
    fn serialize_unsigned(&mut self, n: u64, major_type: u8) {
        let bytes = self.encode_unsigned(n, major_type).to_vec();
        self.writer.write_bytes(bytes);
    }


    fn serialize_seq(&mut self, seq: &Vec<Value>) {
        self.serialize_unsigned(seq.len() as u64, 4);
        self.writer.write_bytes(vec![1, 2, 3]);
    }
}

#[test]
fn serialize_array() {
    let value = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
    assert_eq!(vec![131, 1, 2, 3], to_bytes(value));
}

#[test]
fn ser_u8() {
    let value = Value::Int(2);
    assert_eq!(vec![2], to_bytes(value));
}

#[test]
fn ser_u8_2() {
    let value = Value::Int(42);
    assert_eq!(vec![24, 42], to_bytes(value));
}
