// Generated macro for tests (module)
macro_rules! Depcrate_imp_scalartests {
() => {
// Module: crate::imp::scalar
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn zeroes () { assert_eq ! (adler32 (& []) , 1) ; assert_eq ! (adler32 (& [0]) , 1 | 1 << 16) ; assert_eq ! (adler32 (& [0 , 0]) , 1 | 2 << 16) ; assert_eq ! (adler32 (& [0 ; 100]) , 0x00640001) ; assert_eq ! (adler32 (& [0 ; 1024]) , 0x04000001) ; assert_eq ! (adler32 (& [0 ; 1024 * 1024]) , 0x00f00001) ; } # [test] fn ones () { assert_eq ! (adler32 (& [0xff ; 1024]) , 0x79a6fc2e) ; assert_eq ! (adler32 (& [0xff ; 1024 * 1024]) , 0x8e88ef11) ; } # [test] fn mixed () { assert_eq ! (adler32 (& [1]) , 2 | 2 << 16) ; assert_eq ! (adler32 (& [40]) , 41 | 41 << 16) ; assert_eq ! (adler32 (& [0xA5 ; 1024 * 1024]) , 0xd5009ab1) ; } # [doc = " Example calculation from https://en.wikipedia.org/wiki/Adler-32."] # [test] fn wiki () { assert_eq ! (adler32 (b"Wikipedia") , 0x11E60398) ; } fn adler32 (data : & [u8]) -> u32 { let (a , b) = super :: update (1 , 0 , data) ; u32 :: from (b) << 16 | u32 :: from (a) } }
};
}
