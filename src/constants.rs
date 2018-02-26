#[cfg(feature = "no_std")]
use core::u8::{MAX as MAX_U8_AS_U8};
#[cfg(not(feature = "no_std"))]
use std::u8::{MAX as MAX_U8_AS_U8};
pub const MAX_U8: u64 = MAX_U8_AS_U8 as u64;
