use std::fmt;
use serde::de::{self};
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    U64(u64),
}

impl<'de> de::Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> de::Visitor<'de> for ValueVisitor {
            type Value = Value;
            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("any valid CBOR value")
            }
            #[inline]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Value::U64(v))
                }


        }

        deserializer.deserialize_any(ValueVisitor)
    }
}


impl Value {
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::U64(n) => Some(n),
            _ => None,
        }
    }
}
macro_rules! impl_from {
    ($for_enum:ident, $variant:ident, $for_type:ty) => (
        impl From<$for_type> for $for_enum {
            fn from (v: $for_type) -> $for_enum {
                $for_enum::$variant(v.into())
            }
        }
    )
}
impl_from!(Value, U64, u64);
impl_from!(Value, U64, u8);
