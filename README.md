Serde CBOR Rust - no_std
===============

Serde CBOR Rust is a CBOR Deserializers for Rust which runs in a
[no_std](https://doc.rust-lang.org/book/first-edition/using-rust-without-the-standard-library.html)
environment.

Currently only a deserializer is implemented and it only deserializes u8s and
slices of u8s.

For a fully working implmentation use [pyfisch's
cbor](https://github.com/pyfisch/cbor)


Usage
====

    extern crate cbor_no_std
    use cbor_no_std::from_slice;

    println!("{:?}", from_slice::<Value>(b"\x83\x01\x02\x03").unwrap());
    // => Array(1, 2, 3)
    println!("{:?}", from_slice::<Value>(b"\x01").unwrap());
    // => U64(1)
    println!("{:?}", from_slice::<Value>(b"\x18\x2a").unwrap());
    // => U64(42)
