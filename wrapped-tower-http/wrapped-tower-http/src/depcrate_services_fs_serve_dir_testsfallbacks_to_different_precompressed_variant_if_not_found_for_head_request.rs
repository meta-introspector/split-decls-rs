// Generated macro for fallbacks_to_different_precompressed_variant_if_not_found_for_head_request (function)
macro_rules! Depcrate_services_fs_serve_dir_testsfallbacks_to_different_precompressed_variant_if_not_found_for_head_request {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"fallbacks_to_different_precompressed_variant_if_not_found_for_head_request"}
// Dependencies: {}
# [tokio :: test] async fn fallbacks_to_different_precompressed_variant_if_not_found_for_head_request () { let svc = ServeDir :: new ("../test-files") . precompressed_gzip () . precompressed_br () ; let req = Request :: builder () . uri ("/precompressed_br.txt") . header ("Accept-Encoding" , "gzip,br,deflate") . method (Method :: HEAD) . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert_eq ! (res . headers () ["content-encoding"] , "br") ; assert_eq ! (res . headers () ["content-length"] , "15") ; assert ! (res . into_body () . frame () . await . is_none ()) ; }
};
}
