#[cfg(all(feature = "no_std", not(test)))]
use core::u8::{MAX as MAX_U8_AS_U8};
#[cfg(any(not(feature = "no_std"), test))]
use std::u8::{MAX as MAX_U8_AS_U8};
pub const MAX_U8: usize = MAX_U8_AS_U8 as usize;
