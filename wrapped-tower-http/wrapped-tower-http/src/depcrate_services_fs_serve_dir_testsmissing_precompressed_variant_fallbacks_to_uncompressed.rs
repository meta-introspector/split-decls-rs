// Generated macro for missing_precompressed_variant_fallbacks_to_uncompressed (function)
macro_rules! Depcrate_services_fs_serve_dir_testsmissing_precompressed_variant_fallbacks_to_uncompressed {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"missing_precompressed_variant_fallbacks_to_uncompressed"}
// Dependencies: {}
# [tokio :: test] async fn missing_precompressed_variant_fallbacks_to_uncompressed () { let svc = ServeDir :: new ("../test-files") . precompressed_gzip () ; let request = Request :: builder () . uri ("/missing_precompressed.txt") . header ("Accept-Encoding" , "gzip") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (request) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert ! (res . headers () . get ("content-encoding") . is_none ()) ; let body = res . into_body () . collect () . await . unwrap () . to_bytes () ; let body = String :: from_utf8 (body . to_vec ()) . unwrap () ; assert ! (body . starts_with ("Test file!")) ; }
};
}
