// Generated macro for precompressed_br (function)
macro_rules! Depcrate_services_fs_serve_dir_testsprecompressed_br {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"precompressed_br"}
// Dependencies: {}
# [tokio :: test] async fn precompressed_br () { let svc = ServeDir :: new ("../test-files") . precompressed_br () ; let req = Request :: builder () . uri ("/precompressed.txt") . header ("Accept-Encoding" , "br") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert_eq ! (res . headers () ["content-encoding"] , "br") ; let body = res . into_body () . collect () . await . unwrap () . to_bytes () ; let mut decompressed = Vec :: new () ; BrotliDecompress (& mut & body [..] , & mut decompressed) . unwrap () ; let decompressed = String :: from_utf8 (decompressed . to_vec ()) . unwrap () ; assert ! (decompressed . starts_with ("\"This is a test file!\"")) ; }
};
}
