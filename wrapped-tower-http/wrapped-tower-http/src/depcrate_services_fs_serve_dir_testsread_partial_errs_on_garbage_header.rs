// Generated macro for read_partial_errs_on_garbage_header (function)
macro_rules! Depcrate_services_fs_serve_dir_testsread_partial_errs_on_garbage_header {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"read_partial_errs_on_garbage_header"}
// Dependencies: {}
# [tokio :: test] async fn read_partial_errs_on_garbage_header () { let svc = ServeDir :: new ("..") ; let req = Request :: builder () . uri ("/README.md") . header ("Range" , "bad_format") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: RANGE_NOT_SATISFIABLE) ; let file_contents = std :: fs :: read ("../README.md") . unwrap () ; assert_eq ! (res . headers () ["content-range"] , & format ! ("bytes */{}" , file_contents . len ())) }
};
}
