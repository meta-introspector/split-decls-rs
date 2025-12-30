// Generated macro for read_partial_empty (function)
macro_rules! Depcrate_services_fs_serve_dir_testsread_partial_empty {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"read_partial_empty"}
// Dependencies: {}
# [tokio :: test] async fn read_partial_empty () { let svc = ServeDir :: new ("../test-files") ; let req = Request :: builder () . uri ("/empty.txt") . header ("Range" , "bytes=0-") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: PARTIAL_CONTENT) ; assert_eq ! (res . headers () ["content-length"] , "0") ; assert_eq ! (res . headers () ["content-range"] , "bytes 0-0/0") ; let body = to_bytes (res . into_body ()) . await . ok () . unwrap () ; assert ! (body . is_empty ()) ; }
};
}
