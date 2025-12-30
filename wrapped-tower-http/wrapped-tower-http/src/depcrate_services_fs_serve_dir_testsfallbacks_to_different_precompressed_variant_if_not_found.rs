// Generated macro for fallbacks_to_different_precompressed_variant_if_not_found (function)
macro_rules! Depcrate_services_fs_serve_dir_testsfallbacks_to_different_precompressed_variant_if_not_found {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"fallbacks_to_different_precompressed_variant_if_not_found"}
// Dependencies: {}
# [tokio :: test] async fn fallbacks_to_different_precompressed_variant_if_not_found () { let svc = ServeDir :: new ("../test-files") . precompressed_gzip () . precompressed_br () ; let req = Request :: builder () . uri ("/precompressed_br.txt") . header ("Accept-Encoding" , "gzip,br,deflate") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert_eq ! (res . headers () ["content-encoding"] , "br") ; let body = res . into_body () . collect () . await . unwrap () . to_bytes () ; let mut decompressed = Vec :: new () ; BrotliDecompress (& mut & body [..] , & mut decompressed) . unwrap () ; let decompressed = String :: from_utf8 (decompressed . to_vec ()) . unwrap () ; assert ! (decompressed . starts_with ("Test file")) ; }
};
}
