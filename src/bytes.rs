pub mod bytes {
    #[cfg(all(feature = "no_std", not(test)))]
    use core::str;
    #[cfg(any(not(feature = "no_std"), test))]
    use std::str;
    use value::Value;

    pub fn concat(left: u8, right: u8) -> u8 {
        left << 5 | right
    }

    pub fn major_type(value: u8) -> u8 {
        value >> 5
    }

    pub fn additional_type(value: u8) -> u8 {
        value & 0b00011111
    }

    pub fn to_string(bytes: &[u8]) -> Value {
        match str::from_utf8(&bytes) {
             Ok(s) => Value::String(s.into()),
             Err(_) => unreachable!(),
        }
    }
}
