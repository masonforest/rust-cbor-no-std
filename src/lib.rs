#![feature(lang_items, alloc)]
#![cfg_attr(all(feature = "no_std", not(test)), no_std)]

#![cfg(feature = "no_std")]

#[macro_use] extern crate alloc;
pub mod value;
pub mod io;
pub mod de;
pub mod ser;
pub mod constants;
mod error;
mod bytes;
pub use value::{Value};
pub use ser::{to_bytes};
pub use de::{from_bytes};
