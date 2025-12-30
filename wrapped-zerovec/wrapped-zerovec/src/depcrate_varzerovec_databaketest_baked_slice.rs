// Generated macro for test_baked_slice (function)
macro_rules! Depcrate_varzerovec_databaketest_baked_slice {
() => {
// Module: crate::varzerovec::databake
// Provides: {"test_baked_slice"}
// Dependencies: {}
# [test] fn test_baked_slice () { test_bake ! (& VarZeroSlice < str >, const , crate :: vecs :: VarZeroSlice16 :: new_empty () , zerovec) ; test_bake ! (& VarZeroSlice < str >, const , unsafe { crate :: vecs :: VarZeroSlice16 :: from_bytes_unchecked (b"\x02\0\0\0\0\0\x05\0helloworld") } , zerovec) ; }
};
}
