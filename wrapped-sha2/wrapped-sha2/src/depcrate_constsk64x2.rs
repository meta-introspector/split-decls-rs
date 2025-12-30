// Generated macro for K64X2 (const)
macro_rules! Depcrate_constsK64X2 {
() => {
// Module: crate::consts
// Provides: {"K64X2"}
// Dependencies: {}
# [doc = " Swapped round constants for SHA-512 family of digests"] pub (crate) const K64X2 : [[u64 ; 2] ; 40] = { let mut res = [[0u64 ; 2] ; 40] ; let mut i = 0 ; while i < 16 { res [i] = [K64 [4 * i + 1] , K64 [4 * i]] ; i += 1 ; } res } ;
};
}
