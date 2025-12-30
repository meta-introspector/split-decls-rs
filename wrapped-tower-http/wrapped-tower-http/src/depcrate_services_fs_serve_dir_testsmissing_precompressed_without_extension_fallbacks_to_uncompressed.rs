// Generated macro for missing_precompressed_without_extension_fallbacks_to_uncompressed (function)
macro_rules! Depcrate_services_fs_serve_dir_testsmissing_precompressed_without_extension_fallbacks_to_uncompressed {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"missing_precompressed_without_extension_fallbacks_to_uncompressed"}
// Dependencies: {}
# [tokio :: test] async fn missing_precompressed_without_extension_fallbacks_to_uncompressed () { let svc = ServeDir :: new ("../test-files") . precompressed_gzip () ; let request = Request :: builder () . uri ("/extensionless_precompressed_missing") . header ("Accept-Encoding" , "gzip") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (request) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; assert_eq ! (res . headers () ["content-type"] , "application/octet-stream") ; assert ! (res . headers () . get ("content-encoding") . is_none ()) ; let body = res . into_body () . collect () . await . unwrap () . to_bytes () ; let body = String :: from_utf8 (body . to_vec ()) . unwrap () ; let correct = fs :: read_to_string ("../test-files/extensionless_precompressed_missing") . unwrap () ; assert_eq ! (body , correct) ; }
};
}
