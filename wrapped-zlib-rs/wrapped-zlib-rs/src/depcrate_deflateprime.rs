// Generated macro for prime (function)
macro_rules! Depcrate_deflateprime {
() => {
// Module: crate::deflate
// Provides: {"prime"}
// Dependencies: {}
pub fn prime (stream : & mut DeflateStream , mut bits : i32 , value : i32) -> ReturnCode { debug_assert ! (bits <= 16 , "zlib only supports up to 16 bits here") ; let mut value64 = value as u64 ; let state = & mut stream . state ; if bits < 0 || bits > BitWriter :: BIT_BUF_SIZE as i32 || bits > (core :: mem :: size_of_val (& value) << 3) as i32 { return ReturnCode :: BufError ; } let mut put ; loop { put = BitWriter :: BIT_BUF_SIZE - state . bit_writer . bits_used ; let put = Ord :: min (put as i32 , bits) ; if state . bit_writer . bits_used == 0 { state . bit_writer . bit_buffer = value64 ; } else { state . bit_writer . bit_buffer |= (value64 & ((1 << put) - 1)) << state . bit_writer . bits_used ; } state . bit_writer . bits_used += put as u8 ; state . bit_writer . flush_bits () ; value64 >>= put ; bits -= put ; if bits == 0 { break ; } } ReturnCode :: Ok }
};
}
