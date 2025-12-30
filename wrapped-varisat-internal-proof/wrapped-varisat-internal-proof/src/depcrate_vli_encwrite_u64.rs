// Generated macro for write_u64 (function)
macro_rules! Depcrate_vli_encwrite_u64 {
() => {
// Module: crate::vli_enc
// Provides: {"write_u64"}
// Dependencies: {}
# [doc = " Write an encoded 64 bit number."] pub fn write_u64 (target : & mut impl Write , mut value : u64) -> Result < () , io :: Error > { let bits = (64 - value . leading_zeros ()) as u32 ; let blocks = (bits * (64 / 7)) / 64 ; if value < (1 << (8 * 7)) { value = ((value << 1) | 1) << blocks ; let bytes = unsafe { std :: mem :: transmute :: < u64 , [u8 ; 8] > (value . to_le ()) } ; target . write_all (& bytes [.. (blocks + 1) as usize]) } else { let lo_data = ((value << 1) | 1) << blocks ; let lo_bytes = unsafe { std :: mem :: transmute :: < u64 , [u8 ; 8] > (lo_data . to_le ()) } ; let hi_data = value >> (64 - (blocks + 1)) ; let hi_bytes = unsafe { std :: mem :: transmute :: < u64 , [u8 ; 8] > (hi_data . to_le ()) } ; target . write_all (& lo_bytes) ? ; target . write_all (& hi_bytes [.. (blocks as usize) + 1 - 8]) } }
};
}
