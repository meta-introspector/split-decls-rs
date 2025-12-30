// Generated macro for g (function)
macro_rules! Depcrate_block_apig {
() => {
// Module: crate::block_api
// Provides: {"g"}
// Dependencies: {}
fn g (h : & mut [u64 ; 8] , n : & [u64 ; 8] , m : & [u64 ; 8]) { let mut key = * h ; let mut block = * m ; lps (& mut key , n) ; for c in & C64 { lps (& mut block , & key) ; lps (& mut key , c) ; } for i in 0 .. 8 { h [i] ^= block [i] ^ key [i] ^ m [i] ; } }
};
}
