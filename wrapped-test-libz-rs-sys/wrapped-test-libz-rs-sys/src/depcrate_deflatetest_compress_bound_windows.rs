// Generated macro for test_compress_bound_windows (function)
macro_rules! Depcrate_deflatetest_compress_bound_windows {
() => {
// Module: crate::deflate
// Provides: {"test_compress_bound_windows"}
// Dependencies: {}
# [test] # [cfg_attr (target_endian = "big" , ignore = "we don't support DFLTCC, which changes the bounds in zlib-ng")] fn test_compress_bound_windows () { let source_len = 4294967289 as core :: ffi :: c_ulong ; assert_eq_rs_ng ! ({ compressBound (source_len as _) }) ; }
};
}
