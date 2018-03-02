#![cfg(feature = "no_std")]
use alloc::vec::Vec;

pub trait Writer {
    fn write_byte(&mut self, byte: u8);
    fn write_bytes(&mut self, bytes: Vec<u8>);
}

pub struct VecWriter<'a> {
    pub output: &'a mut Vec<u8>
}

impl<'a> Writer for VecWriter<'a> {
    fn write_byte(&mut self, byte: u8) {
        self.write_bytes(vec![byte])
    }

    fn write_bytes(&mut self, mut bytes: Vec<u8>) {
        self.output.append(&mut bytes)
    }
}

pub trait Reader {
    fn read_byte(&mut self) -> u8;
    fn skip_byte(&mut self);
    fn peek_byte(&mut self) -> u8;
    fn read_n_bytes(&mut self, n: usize) -> Vec<u8>;
}

pub struct VecReader {
    pub input: Vec<u8>
}

impl Reader for VecReader {
    fn read_byte(&mut self) -> u8 {
        match self.input.clone().split_first() {
            Some((byte, rest)) => {
                self.input = rest.to_vec();
                *byte
            }
            None => 0
        }
    }

    fn skip_byte(&mut self) {
        let input = self.input.clone();
        match input.split_first() {
            Some((_, rest)) => {
                self.input = rest.to_vec();
            }
            None => ()
        };
    }

    fn peek_byte(&mut self) -> u8 {
        match self.input.first() {
            Some(byte) => *byte,
            None => 0
        }
    }

    fn read_n_bytes(&mut self, n: usize) -> Vec<u8> {
        let input = self.input.clone();
        let (bytes, rest) = input.split_at(n);
        self.input = rest.to_vec();
        bytes.to_vec()
    }
}
