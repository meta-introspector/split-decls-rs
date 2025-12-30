// Generated macro for deflate_bound_gzip_header_basic (function)
macro_rules! Depcrate_deflatedeflate_bound_gzip_header_basic {
() => {
// Module: crate::deflate
// Provides: {"deflate_bound_gzip_header_basic"}
// Dependencies: {}
# [test] fn deflate_bound_gzip_header_basic () { deflate_bound_gzip_header_help ((DeflateConfig :: default () , 1234 , CString :: from_vec_with_nul ("extra\0" . as_bytes () . to_vec ()) . unwrap () , CString :: from_vec_with_nul ("name\0" . as_bytes () . to_vec ()) . unwrap () , CString :: from_vec_with_nul ("comment\0" . as_bytes () . to_vec ()) . unwrap () ,)) ; }
};
}
