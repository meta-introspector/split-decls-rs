// Generated macro for only_precompressed_variant_existing (function)
macro_rules! Depcrate_services_fs_serve_dir_testsonly_precompressed_variant_existing {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"only_precompressed_variant_existing"}
// Dependencies: {}
# [tokio :: test] async fn only_precompressed_variant_existing () { let svc = ServeDir :: new ("../test-files") . precompressed_gzip () ; let request = Request :: builder () . uri ("/only_gzipped.txt") . body (Body :: empty ()) . unwrap () ; let res = svc . clone () . oneshot (request) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: NOT_FOUND) ; let request = Request :: builder () . uri ("/only_gzipped.txt") . header ("Accept-Encoding" , "gzip") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (request) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert_eq ! (res . headers () ["content-encoding"] , "gzip") ; let body = res . into_body () . collect () . await . unwrap () . to_bytes () ; let mut decoder = GzDecoder :: new (& body [..]) ; let mut decompressed = String :: new () ; decoder . read_to_string (& mut decompressed) . unwrap () ; assert ! (decompressed . starts_with ("\"This is a test file\"")) ; }
};
}
