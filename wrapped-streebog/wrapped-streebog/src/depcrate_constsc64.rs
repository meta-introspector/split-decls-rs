// Generated macro for C64 (const)
macro_rules! Depcrate_constsC64 {
() => {
// Module: crate::consts
// Provides: {"C64"}
// Dependencies: {}
# [doc = " Iteration constants represented as `[u64; 8]`"] pub (crate) const C64 : [[u64 ; 8] ; 12] = { let mut res = [[0u64 ; 8] ; 12] ; let mut i = 0 ; let mut buf = [0u8 ; 8] ; while i < 12 { let mut j = 0 ; while j < 8 { let mut k = 0 ; while k < 8 { buf [k] = C [i] [8 * j + k] ; k += 1 ; } res [i] [j] = u64 :: from_le_bytes (buf) ; j += 1 ; } i += 1 ; } res } ;
};
}
