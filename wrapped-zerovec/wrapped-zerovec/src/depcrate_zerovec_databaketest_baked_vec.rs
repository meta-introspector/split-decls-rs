// Generated macro for test_baked_vec (function)
macro_rules! Depcrate_zerovec_databaketest_baked_vec {
() => {
// Module: crate::zerovec::databake
// Provides: {"test_baked_vec"}
// Dependencies: {}
# [test] fn test_baked_vec () { test_bake ! (ZeroVec < u32 >, const , crate :: ZeroVec :: new () , zerovec) ; test_bake ! (ZeroVec < u32 >, const , unsafe { crate :: ZeroVec :: from_bytes_unchecked (b"\x02\x01\0\x16\0M\x01\\") } , zerovec) ; }
};
}
