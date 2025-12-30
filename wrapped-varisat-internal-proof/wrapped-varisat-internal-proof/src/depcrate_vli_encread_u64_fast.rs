// Generated macro for read_u64_fast (function)
macro_rules! Depcrate_vli_encread_u64_fast {
() => {
// Module: crate::vli_enc
// Provides: {"read_u64_fast"}
// Dependencies: {}
# [doc = " Read an encoded 64 bit number, if at least 16 bytes lookahead are available."] fn read_u64_fast (bytes : & [u8 ; 16]) -> (u64 , usize) { let lo_data = u64 :: from_le (unsafe { std :: mem :: transmute :: < [u8 ; 8] , u64 > (* < & [u8 ; 8] > :: try_from (& bytes [.. 8]) . unwrap ()) }) ; let len = (lo_data | (1 << 9)) . trailing_zeros () + 1 ; if len <= 8 { let result = lo_data & (! 0u64 >> (64 - 8 * len)) ; let result = result >> len ; (result , len as usize) } else { let hi_data = u64 :: from_le (unsafe { std :: mem :: transmute :: < [u8 ; 8] , u64 > (* < & [u8 ; 8] > :: try_from (& bytes [8 ..]) . unwrap ()) }) ; let hi_data = hi_data & (! 0u64 >> (64 - 8 * (len - 8))) ; let result = (lo_data >> len) | (hi_data << (64 - len)) ; (result as u64 , len as usize) } }
};
}
