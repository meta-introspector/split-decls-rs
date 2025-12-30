// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl Digest { # [doc = " Returns the 160 bit (20 byte) digest as a byte array."] pub fn bytes (& self) -> [u8 ; DIGEST_LENGTH] { [(self . data . state [0] >> 24) as u8 , (self . data . state [0] >> 16) as u8 , (self . data . state [0] >> 8) as u8 , (self . data . state [0] >> 0) as u8 , (self . data . state [1] >> 24) as u8 , (self . data . state [1] >> 16) as u8 , (self . data . state [1] >> 8) as u8 , (self . data . state [1] >> 0) as u8 , (self . data . state [2] >> 24) as u8 , (self . data . state [2] >> 16) as u8 , (self . data . state [2] >> 8) as u8 , (self . data . state [2] >> 0) as u8 , (self . data . state [3] >> 24) as u8 , (self . data . state [3] >> 16) as u8 , (self . data . state [3] >> 8) as u8 , (self . data . state [3] >> 0) as u8 , (self . data . state [4] >> 24) as u8 , (self . data . state [4] >> 16) as u8 , (self . data . state [4] >> 8) as u8 , (self . data . state [4] >> 0) as u8 ,] } }
};
}
