// Generated macro for zlib_rs_is_not_zlib_ng (function)
macro_rules! Depcratezlib_rs_is_not_zlib_ng {
() => {
// Module: crate
// Provides: {"zlib_rs_is_not_zlib_ng"}
// Dependencies: {}
# [doc = " a bit of a sanity check that nothing weird happened with symbol resolution"] # [cfg (not (miri))] # [cfg (test)] # [test] fn zlib_rs_is_not_zlib_ng () { use std :: ffi :: CStr ; unsafe { let rs = CStr :: from_ptr (libz_rs_sys :: zlibVersion ()) ; assert ! (rs . to_str () . unwrap () . contains ("zlib-rs")) ; let ng = CStr :: from_ptr (libz_sys :: zlibVersion ()) ; assert ! (ng . to_str () . unwrap () . contains ("zlib-ng")) ; } }
};
}
