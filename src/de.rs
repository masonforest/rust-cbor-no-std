use std::ops::{Neg, AddAssign, MulAssign};


use serde::de::{self, Deserialize, Visitor};

use error::{Error, Result};
use value::Value;

pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(input: &'de [u8]) -> Self {
        Deserializer { input: input }
    }
}

pub fn from_slice<'a, T>(s: &'a [u8]) -> Result<T>
    where T: Deserialize<'a>
{
    let mut deserializer = Deserializer::from_slice(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

impl<'de> Deserializer<'de> {
    fn next_byte(&mut self) -> Result<u8> {
        match self.input.split_first() {
            Some((byte, rest)) => {
                self.input = rest;
                Ok(*byte)
            }
            None => Err(Error::Eof)
        }
    }

    fn skip_byte(&mut self) -> Result<()> {
        match self.input.split_first() {
            Some((_, rest)) => {
                self.input = rest;
                Ok(())
            }
            None => Err(Error::Eof)
        }
    }

    fn peek_byte(&mut self) -> Result<u8> {
        match self.input.first() {
            Some(byte) => Ok(*byte),
            None => Err(Error::Eof)
        }
    }

    fn next_n_bytes(&mut self, n: usize) -> &'de [u8] {
        let (bytes, rest) = self.input.split_at(n);
        self.input = rest;
        &bytes
    }

    fn parse_bool(&mut self) -> Result<bool> {
        unimplemented!()
    }


    fn parse_value<V>(&mut self, visitor: V) -> Result<V::Value>
            where
        V: de::Visitor<'de>,
    {
        let byte = self.peek_byte()?;
        let value = match byte {
            0x00...0x17 => {
                let value = self.next_byte()?;
                Ok(Value::U64(byte.into()))
            },
            0x18 => {
                self.skip_byte();
                let value = self.next_byte()?;
                Ok(Value::U64(value.into()))
            },
            0x80...0x97 => {
                self.skip_byte();
                let n = byte as usize - 0x80;
                Ok(Value::Array(self.next_n_bytes(n)))
            },
            _ => unreachable!(),
        };

        self.visit_value(visitor, value?)
    }

    fn visit_value<V>(&mut self, visitor: V, value: Value<'de>) -> Result<V::Value>
            where
        V: de::Visitor<'de> {
        match value {
            Value::U64(n) => visitor.visit_u64(n.into()),
            Value::Array(n) => visitor.visit_borrowed_bytes(n),
            _ => Err(Error::Eof),
        }
    }

    fn parse_array(&mut self, mut len: usize) -> Result<Value>
        {
            Ok(self.next_n_bytes(len).into())
    }
    fn parse_unsigned<T>(&mut self) -> Result<T>
        where T: AddAssign<T> + MulAssign<T> + From<u8>
    {
        unimplemented!()
    }

    fn parse_signed<T>(&mut self) -> Result<T>
        where T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8>
    {
        unimplemented!()
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        unimplemented!()
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        self.parse_value(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_i8(self.parse_signed()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_i16(self.parse_signed()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_i32(self.parse_signed()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_i64(self.parse_signed()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_u8(self.parse_unsigned()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_u16(self.parse_unsigned()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_u32(self.parse_unsigned()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_u64(self.parse_unsigned()?)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V
    ) -> Result<V::Value>
        where V: Visitor<'de>
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V
    ) -> Result<V::Value>
        where V: Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        visitor: V
    ) -> Result<V::Value>
        where V: Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V
    ) -> Result<V::Value>
        where V: Visitor<'de>
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V
    ) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V
    ) -> Result<V::Value>
        where V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V
    ) -> Result<V::Value>
        where V: Visitor<'de>
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V
    ) -> Result<V::Value>
        where V: Visitor<'de>
    {
        self.deserialize_any(visitor)
    }
}

#[test]
fn test_u64() {
    let expected: Value = Value::Array(&[1, 2, 3]);
    assert_eq!(expected, from_slice(b"\x83\x01\x02\x03").unwrap());
}
