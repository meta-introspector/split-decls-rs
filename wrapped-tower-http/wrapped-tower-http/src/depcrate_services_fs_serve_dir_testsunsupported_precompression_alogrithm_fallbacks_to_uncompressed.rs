// Generated macro for unsupported_precompression_alogrithm_fallbacks_to_uncompressed (function)
macro_rules! Depcrate_services_fs_serve_dir_testsunsupported_precompression_alogrithm_fallbacks_to_uncompressed {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"unsupported_precompression_alogrithm_fallbacks_to_uncompressed"}
// Dependencies: {}
# [tokio :: test] async fn unsupported_precompression_alogrithm_fallbacks_to_uncompressed () { let svc = ServeDir :: new ("../test-files") . precompressed_gzip () ; let request = Request :: builder () . uri ("/precompressed.txt") . header ("Accept-Encoding" , "br") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (request) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert ! (res . headers () . get ("content-encoding") . is_none ()) ; let body = res . into_body () . collect () . await . unwrap () . to_bytes () ; let body = String :: from_utf8 (body . to_vec ()) . unwrap () ; assert ! (body . starts_with ("\"This is a test file!\"")) ; }
};
}
