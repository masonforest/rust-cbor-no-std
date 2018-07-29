// extern crate core;
//
// #[cfg(feature = "std")]
// use std::fmt::{self, Display};
// #[cfg(not(feature = "std"))]
// use self::core::fmt::{self, Display};
// use serde::{de};
//
// pub type Result<T> = core::result::Result<T, Error>;
// #[derive(Clone, Debug, PartialEq)]
// pub struct Error {
//     pub err: ErrorImpl,
// }
//
// #[cfg(any(feature = "std"))]
// type ErrorImpl = Box<str>;
// #[cfg(not(feature = "std"))]
// type ErrorImpl = ();
//
// impl de::Error for Error {
//     #[cfg(feature = "std")]
//     fn custom<T>(msg: T) -> Self
//     where
//         T: Display,
//     {
//         Error {
//             err: msg.to_string().into_boxed_str(),
//         }
//     }
//
//     #[cfg(not(feature = "std"))]
//     fn custom<T>(msg: T) -> Self
//     where
//         T: Display,
//     {
//         let _ = msg;
//         Error { err: () }
//     }
// }
//
// impl Display for Error {
//     #[cfg(any(feature = "std"))]
//     fn fmt(&self, formatter: &mut fmt::Formatter) -> core::Result<(), fmt::Error> {
//         formatter.write_str(&self.err)
//     }
//
//     #[cfg(not(feature = "std"))]
//     fn fmt(&self, formatter: &mut fmt::Formatter) -> core::result::Result<(), fmt::Error> {
//         formatter.write_str("Serde deserialization error")
//     }
// }
