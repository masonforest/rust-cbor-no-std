use alloc::vec::Vec;
use value::Value;
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

impl<R> Deserializer<R> where R: Reader {
    fn from_vec_reader(bytes: R) -> Deserializer<R> {
        Deserializer{reader: bytes}
    }

    fn parse_value(&mut self) -> Value
    {
        let byte = self.reader.peek_byte();
        match byte {
            0x00...0x18 => self.parse_u8(),
            0x80...0x97 => self.parse_array(),
            _ => unreachable!(),
        }
    }

    fn parse_array(&mut self) -> Value {
        let len = self.reader.read_byte() as usize - 0x80;
        let values = (0..len).map(|_| self.parse_value()).collect();
        Value::Array(values)
    }

    fn parse_u8(&mut self) -> Value {
        if self.reader.peek_byte() == 0x18 {
            self.reader.skip_byte();
        }

        Value::Int(self.reader.read_byte().into())
    }
}


#[test]
fn deserialize_array() {
    let expected: Value = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
    assert_eq!(expected, from_bytes(vec![131, 1, 2, 3]));
}

#[test]
fn deserialize_u8() {
    let expected: Value = Value::Int(1);
    assert_eq!(expected, from_bytes(vec![1]));
}

#[test]
fn deserialize_u8_2() {
    let expected: Value = Value::Int(42);
    assert_eq!(expected, from_bytes(vec![24, 42]));
}
