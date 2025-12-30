// Generated macro for precompressed_deflate (function)
macro_rules! Depcrate_services_fs_serve_dir_testsprecompressed_deflate {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"precompressed_deflate"}
// Dependencies: {}
# [tokio :: test] async fn precompressed_deflate () { let svc = ServeDir :: new ("../test-files") . precompressed_deflate () ; let request = Request :: builder () . uri ("/precompressed.txt") . header ("Accept-Encoding" , "deflate,br") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (request) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert_eq ! (res . headers () ["content-encoding"] , "deflate") ; let body = res . into_body () . collect () . await . unwrap () . to_bytes () ; let mut decoder = DeflateDecoder :: new (& body [..]) ; let mut decompressed = String :: new () ; decoder . read_to_string (& mut decompressed) . unwrap () ; assert ! (decompressed . starts_with ("\"This is a test file!\"")) ; }
};
}
