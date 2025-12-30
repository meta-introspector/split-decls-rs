// Generated macro for test_baked_slice (function)
macro_rules! Depcrate_zerovec_databaketest_baked_slice {
() => {
// Module: crate::zerovec::databake
// Provides: {"test_baked_slice"}
// Dependencies: {}
# [test] fn test_baked_slice () { test_bake ! (& ZeroSlice < u32 >, const , crate :: ZeroSlice :: new_empty () , zerovec) ; test_bake ! (& ZeroSlice < u32 >, const , unsafe { crate :: ZeroSlice :: from_bytes_unchecked (b"\x02\x01\0\x16\0M\x01\\") } , zerovec) ; }
};
}
