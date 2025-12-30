// Generated macro for prime (function)
macro_rules! Depcrate_inflateprime {
() => {
// Module: crate::inflate
// Provides: {"prime"}
// Dependencies: {}
pub fn prime (stream : & mut InflateStream , bits : i32 , value : i32) -> ReturnCode { if bits == 0 { } else if bits < 0 { stream . state . bit_reader . init_bits () ; } else if bits > 16 || stream . state . bit_reader . bits_in_buffer () + bits as u8 > 32 { return ReturnCode :: StreamError ; } else { stream . state . bit_reader . prime (bits as u8 , value as u64) ; } ReturnCode :: Ok }
};
}
