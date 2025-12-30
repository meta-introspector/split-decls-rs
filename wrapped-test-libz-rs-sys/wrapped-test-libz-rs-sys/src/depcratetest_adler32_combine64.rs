// Generated macro for test_adler32_combine64 (function)
macro_rules! Depcratetest_adler32_combine64 {
() => {
// Module: crate
// Provides: {"test_adler32_combine64"}
// Dependencies: {}
# [test] fn test_adler32_combine64 () { use libz_rs_sys :: z_off64_t ; let a = 0x98AB_CDEF ; let b = 0x1234_5678 ; assert_eq_rs_ng ! ({ adler32_combine64 (a , b , - 1 as z_off64_t) }) ; assert_eq_rs_ng ! ({ adler32_combine64 (a , b , 0 as z_off64_t) }) ; assert_eq_rs_ng ! ({ adler32_combine64 (a , b , 32 as z_off64_t) }) ; assert_eq_rs_ng ! ({ adler32_combine64 (a , b , i64 :: MAX as z_off64_t) }) ; }
};
}
