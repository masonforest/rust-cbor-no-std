#![feature(lang_items, alloc)]
#![cfg_attr(any(feature = "no_std"), no_std)]

#[macro_use] extern crate alloc;
pub mod value;
pub mod io;
pub mod de;
pub mod ser;
pub mod constants;
mod error;
pub use value::{Value};

#[cfg(all(feature = "no_std", not(test)))]
#[lang = "eh_personality"] extern fn eh_personality() {}

#[cfg(all(feature = "no_std", not(test)))]
#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
   loop {}
}
