// Generated macro for precompresed_head_request (function)
macro_rules! Depcrate_services_fs_serve_dir_testsprecompresed_head_request {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"precompresed_head_request"}
// Dependencies: {}
# [tokio :: test] async fn precompresed_head_request () { let svc = ServeDir :: new ("../test-files") . precompressed_gzip () ; let req = Request :: builder () . uri ("/precompressed.txt") . header ("Accept-Encoding" , "gzip") . method (Method :: HEAD) . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert_eq ! (res . headers () ["content-encoding"] , "gzip") ; assert_eq ! (res . headers () ["content-length"] , "59") ; assert ! (res . into_body () . frame () . await . is_none ()) ; }
};
}
