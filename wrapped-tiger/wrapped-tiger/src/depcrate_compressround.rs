// Generated macro for round (function)
macro_rules! Depcrate_compressround {
() => {
// Module: crate::compress
// Provides: {"round"}
// Dependencies: {}
# [inline (always)] fn round (a : & mut u64 , b : & mut u64 , c : & mut u64 , x : & u64 , mul : u8) { * c ^= * x ; let c2 : [u8 ; 8] = c . to_le_bytes () ; let a2 = TABLES [0] [usize :: from (c2 [0])] ^ TABLES [1] [usize :: from (c2 [2])] ^ TABLES [2] [usize :: from (c2 [4])] ^ TABLES [3] [usize :: from (c2 [6])] ; let b2 = TABLES [3] [usize :: from (c2 [1])] ^ TABLES [2] [usize :: from (c2 [3])] ^ TABLES [1] [usize :: from (c2 [5])] ^ TABLES [0] [usize :: from (c2 [7])] ; * a = a . wrapping_sub (a2) ; * b = b . wrapping_add (b2) . wrapping_mul (u64 :: from (mul)) ; }
};
}
