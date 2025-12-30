// Generated macro for head_request (function)
macro_rules! Depcrate_services_fs_serve_dir_testshead_request {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"head_request"}
// Dependencies: {}
# [tokio :: test] async fn head_request () { let svc = ServeDir :: new ("../test-files") ; let req = Request :: builder () . uri ("/precompressed.txt") . method (Method :: HEAD) . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; assert_eq ! (res . headers () ["content-length"] , "23") ; assert ! (res . into_body () . frame () . await . is_none ()) ; }
};
}
