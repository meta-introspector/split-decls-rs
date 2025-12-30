// Generated macro for lps (function)
macro_rules! Depcrate_block_apilps {
() => {
// Module: crate::block_api
// Provides: {"lps"}
// Dependencies: {}
# [inline (always)] fn lps (h : & mut [u64 ; 8] , n : & [u64 ; 8]) { for i in 0 .. 8 { h [i] ^= n [i] ; } let mut buf = [0u64 ; 8] ; # [allow (clippy :: needless_range_loop)] for i in 0 .. 8 { for j in 0 .. 8 { let idx = ((h [j] >> (8 * i)) & 0xff) as usize ; buf [i] ^= SHUFFLED_LIN_TABLE [j] [idx] ; } } * h = buf ; }
};
}
