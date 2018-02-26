pub mod bytes {
    pub fn concat(left: u8, right: u8) -> u8 {
        left << 5 | right
    }
}
