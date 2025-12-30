// Generated macro for assert_eq_rs_ng (macro)
macro_rules! Depcrateassert_eq_rs_ng {
() => {
// Module: crate
// Provides: {"assert_eq_rs_ng"}
// Dependencies: {}
# [cfg (test)] # [macro_export] macro_rules ! assert_eq_rs_ng { ($ tt : tt) => { { # [cfg (not (miri))] # [allow (clippy :: macro_metavars_in_unsafe)] # [allow (unused_braces)] # [allow (unused_unsafe)] let _ng = unsafe { use libz_sys ::*; extern "C" { # [allow (unused)] fn inflateCodesUsed (strm : * mut z_stream) -> core :: ffi :: c_ulong ; # [allow (unused)] fn deflateGetDictionary (strm : * const z_stream , dictionary : * mut core :: ffi :: c_uchar , dictLength : * mut core :: ffi :: c_uint ,) -> core :: ffi :: c_int ; # [allow (unused)] fn inflateGetDictionary (strm : * const z_stream , dictionary : * mut core :: ffi :: c_uchar , dictLength : * mut core :: ffi :: c_uint ,) -> core :: ffi :: c_int ; # [allow (unused)] fn crc32_z (crc : core :: ffi :: c_ulong , buf : * const Bytef , len : libz_rs_sys :: size_t ,) -> core :: ffi :: c_ulong ; # [allow (unused)] fn crc32_combine64 (crc1 : core :: ffi :: c_ulong , crc2 : core :: ffi :: c_ulong , len2 : libz_rs_sys :: z_off64_t ,) -> core :: ffi :: c_ulong ; # [allow (unused)] fn adler32_z (crc : core :: ffi :: c_ulong , buf : * const Bytef , len : libz_rs_sys :: size_t ,) -> core :: ffi :: c_ulong ; # [allow (unused)] fn adler32_combine64 (adler1 : core :: ffi :: c_ulong , adler2 : core :: ffi :: c_ulong , len2 : libz_rs_sys :: z_off64_t ,) -> core :: ffi :: c_ulong ; # [allow (unused)] fn get_crc_table () -> * const [u32 ; 256] ; } $ tt } ; # [allow (clippy :: macro_metavars_in_unsafe)] # [allow (unused_braces)] # [allow (unused_unsafe)] let _rs = unsafe { use libz_rs_sys ::*; $ tt } ; # [cfg (not (miri))] assert_eq ! (_rs , _ng) ; _rs } } ; }
};
}
