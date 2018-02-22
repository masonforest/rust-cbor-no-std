#![feature(lang_items)]
#![cfg_attr(feature = "no_std", no_std)]
extern crate serde;

pub mod value;
pub mod de;
mod error;
#[cfg(all(feature = "no_std", not(test)))]
#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}
