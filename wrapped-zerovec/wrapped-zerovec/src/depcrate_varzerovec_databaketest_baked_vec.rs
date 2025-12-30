// Generated macro for test_baked_vec (function)
macro_rules! Depcrate_varzerovec_databaketest_baked_vec {
() => {
// Module: crate::varzerovec::databake
// Provides: {"test_baked_vec"}
// Dependencies: {}
# [test] fn test_baked_vec () { test_bake ! (VarZeroVec < str >, const , crate :: vecs :: VarZeroVec16 :: new () , zerovec) ; test_bake ! (VarZeroVec < str >, const , unsafe { crate :: vecs :: VarZeroVec16 :: from_bytes_unchecked (b"\x02\0\0\0\0\0\x05\0helloworld") } , zerovec) ; }
};
}
