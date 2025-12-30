// Generated macro for test_compress_bound (function)
macro_rules! Depcrate_deflatetest_compress_bound {
() => {
// Module: crate::deflate
// Provides: {"test_compress_bound"}
// Dependencies: {}
# [test] # [cfg_attr (target_endian = "big" , ignore = "we don't support DFLTCC, which changes the bounds in zlib-ng")] fn test_compress_bound () { :: quickcheck :: quickcheck (test as fn (_) -> _) ; fn test (source_len : core :: ffi :: c_ulong) -> bool { assert_eq_rs_ng ! ({ compressBound (source_len as _) }) ; true } }
};
}
