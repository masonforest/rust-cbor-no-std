extern crate core;
use serde::de::{self};
use self::core::fmt::{Result as FormatterResult, Formatter};
#[derive(Clone, Debug, PartialEq)]
pub enum Value<'a> {
    U64(u64),
    Array(&'a [u8]),
}

impl<'de> de::Deserialize<'de> for Value<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value<'de>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> de::Visitor<'de> for ValueVisitor {
            type Value = Value<'de>;

            fn expecting(&self, fmt: &mut Formatter) -> FormatterResult {
                fmt.write_str("any valid CBOR value")
            }
            #[inline]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Value::U64(v))
                }
            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E> {
                    Ok(Value::Array(v))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}


impl<'a> Value<'a> {
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::U64(n) => Some(n),
            _ => None,
        }
    }
}

macro_rules! impl_from {
    ($for_enum:ident, $variant:ident, $for_type:ty) => (
        impl<'a> From<$for_type> for $for_enum<'a> {
            fn from (v: $for_type) -> $for_enum<'a> {
                $for_enum::$variant(v.into())
            }
        }
    )
}

impl_from!(Value, U64, u64);
impl_from!(Value, U64, u8);
impl_from!(Value, Array, &'a [u8]);

// impl<'a> From<&[u8]> for Value<'a> {
//     fn from (v: &[u8]) -> Value<'a> {
//         $for_enum::$variant(v.into())
//     }
// }
