// Generated macro for missing_precompressed_variant_fallbacks_to_uncompressed_for_head_request (function)
macro_rules! Depcrate_services_fs_serve_dir_testsmissing_precompressed_variant_fallbacks_to_uncompressed_for_head_request {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"missing_precompressed_variant_fallbacks_to_uncompressed_for_head_request"}
// Dependencies: {}
# [tokio :: test] async fn missing_precompressed_variant_fallbacks_to_uncompressed_for_head_request () { let svc = ServeDir :: new ("../test-files") . precompressed_gzip () ; let request = Request :: builder () . uri ("/missing_precompressed.txt") . header ("Accept-Encoding" , "gzip") . method (Method :: HEAD) . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (request) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert_eq ! (res . headers () ["content-length"] , "11") ; assert ! (res . headers () . get ("content-encoding") . is_none ()) ; assert ! (res . into_body () . frame () . await . is_none ()) ; }
};
}
