// Generated macro for precompressed_gzip (function)
macro_rules! Depcrate_services_fs_serve_dir_testsprecompressed_gzip {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"precompressed_gzip"}
// Dependencies: {}
# [tokio :: test] async fn precompressed_gzip () { let svc = ServeDir :: new ("../test-files") . precompressed_gzip () ; let req = Request :: builder () . uri ("/precompressed.txt") . header ("Accept-Encoding" , "gzip") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert_eq ! (res . headers () ["content-encoding"] , "gzip") ; let body = res . into_body () . collect () . await . unwrap () . to_bytes () ; let mut decoder = GzDecoder :: new (& body [..]) ; let mut decompressed = String :: new () ; decoder . read_to_string (& mut decompressed) . unwrap () ; assert ! (decompressed . starts_with ("\"This is a test file!\"")) ; }
};
}
