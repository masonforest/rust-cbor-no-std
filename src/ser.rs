use constants::*;
use alloc::vec::Vec;
use value::{Value};
use io::{Writer, VecWriter};

pub trait Serializer {
    fn serialize(self);
    fn serialize_unsigned(self, n: u64);
}

pub trait Serialize {
    fn serialize<S>(&mut self, serializer: S) where S: Serializer;
}

pub struct VecSerializer<'a>  {
    pub writer: VecWriter<'a>,
}

struct VecWrapper<'a> {
    vec: &'a mut Vec<u8>
}

fn push_one<'a>(a: &'a mut VecWriter<'a>) -> &'a VecWriter<'a> {
    a.output.push(1);
    a
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
}

impl<'a> Serializer for VecSerializer<'a> {
    fn serialize(self) {
        println!("Serializin!");
    }

    fn serialize_unsigned(mut self, n: u64) {
        let bytes = match n {
            n @ 0 ... 23 => vec![n as u8],
            n @ 24 ... MAX_U8 => vec![24, n as u8],
            _ => unreachable!()
        };
        println!("Serializin unsigned!{:?}\n", n);
        self.writer.write_bytes(bytes);
    }
}

#[test]
fn test_u8() {
    let mut value: Value = Value::Int(2);
    assert_eq!(vec![2], to_bytes(value));
}

#[test]
fn test_u8_2() {
    let mut value: Value = Value::Int(42);
    assert_eq!(vec![24, 42], to_bytes(value));
}
