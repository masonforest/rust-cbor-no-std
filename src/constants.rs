#[cfg(all(feature = "no_std", not(test)))]
pub use core::u8::{MAX as MAX_U8_AS_U8};
#[cfg(any(not(feature = "no_std"), test))]
use std::u8::{MAX as MAX_U8_AS_U8};
#[cfg(all(feature = "no_std", not(test)))]
use core::u16::{MAX as MAX_U16_AS_U16};
#[cfg(any(not(feature = "no_std"), test))]
use std::u16::{MAX as MAX_U16_AS_U16};
#[cfg(all(feature = "no_std", not(test)))]
use core::u32::{MAX as MAX_U32_AS_U32};
#[cfg(any(not(feature = "no_std"), test))]
use std::u32::{MAX as MAX_U32_AS_U32};
#[cfg(all(feature = "no_std", not(test)))]
use core::u64::{MAX as MAX_U64_AS_U64};
#[cfg(any(not(feature = "no_std"), test))]
use std::u64::{MAX as MAX_U64_AS_U64};

pub const MAX_U8: usize = MAX_U8_AS_U8 as usize;
pub const MIN_U16: usize = MAX_U8 + 1 as usize;
pub const MAX_U16: usize = MAX_U16_AS_U16 as usize;
pub const MIN_U32: usize = MAX_U16 + 1 as usize;
pub const MAX_U32: usize = MAX_U32_AS_U32 as usize;
pub const MIN_U64: usize = MAX_U32 + 1 as usize;
pub const MAX_U64: usize = MAX_U64_AS_U64 as usize;
