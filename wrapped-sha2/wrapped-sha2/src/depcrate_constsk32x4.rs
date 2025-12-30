// Generated macro for K32X4 (static)
macro_rules! Depcrate_constsK32X4 {
() => {
// Module: crate::consts
// Provides: {"K32X4"}
// Dependencies: {}
# [doc = " Swapped round constants for SHA-256 family of digests"] pub (crate) static K32X4 : [[u32 ; 4] ; 16] = { let mut res = [[0u32 ; 4] ; 16] ; let mut i = 0 ; while i < 16 { res [i] = [K32 [4 * i + 3] , K32 [4 * i + 2] , K32 [4 * i + 1] , K32 [4 * i]] ; i += 1 ; } res } ;
};
}
