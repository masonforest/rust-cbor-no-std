#[macro_use] extern crate serde_derive;
#[macro_use]
extern crate serde;
macro_rules! try {
    ($e:expr) => {
        match $e {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(err) => {
                return ::std::result::Result::Err(err)
            }
        }
    }
}

pub mod de;
mod error;
