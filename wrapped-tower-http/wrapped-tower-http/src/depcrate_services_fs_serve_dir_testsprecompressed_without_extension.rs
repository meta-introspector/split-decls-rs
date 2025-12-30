// Generated macro for precompressed_without_extension (function)
macro_rules! Depcrate_services_fs_serve_dir_testsprecompressed_without_extension {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"precompressed_without_extension"}
// Dependencies: {}
# [tokio :: test] async fn precompressed_without_extension () { let svc = ServeDir :: new ("../test-files") . precompressed_gzip () ; let request = Request :: builder () . uri ("/extensionless_precompressed") . header ("Accept-Encoding" , "gzip") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (request) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; assert_eq ! (res . headers () ["content-type"] , "application/octet-stream") ; assert_eq ! (res . headers () ["content-encoding"] , "gzip") ; let body = res . into_body () . collect () . await . unwrap () . to_bytes () ; let mut decoder = GzDecoder :: new (& body [..]) ; let mut decompressed = String :: new () ; decoder . read_to_string (& mut decompressed) . unwrap () ; let correct = fs :: read_to_string ("../test-files/extensionless_precompressed") . unwrap () ; assert_eq ! (decompressed , correct) ; }
};
}
