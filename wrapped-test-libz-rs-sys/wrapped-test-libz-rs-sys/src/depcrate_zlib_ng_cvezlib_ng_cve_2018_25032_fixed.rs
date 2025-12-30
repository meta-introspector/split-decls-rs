// Generated macro for zlib_ng_cve_2018_25032_fixed (function)
macro_rules! Depcrate_zlib_ng_cvezlib_ng_cve_2018_25032_fixed {
() => {
// Module: crate::zlib_ng_cve
// Provides: {"zlib_ng_cve_2018_25032_fixed"}
// Dependencies: {}
# [test] # [cfg_attr (miri , ignore)] fn zlib_ng_cve_2018_25032_fixed () { const FIXED : & str = include_str ! ("test-data/zlib-ng/CVE-2018-25032/fixed.txt") ; cve_test (FIXED . as_bytes ()) }
};
}
