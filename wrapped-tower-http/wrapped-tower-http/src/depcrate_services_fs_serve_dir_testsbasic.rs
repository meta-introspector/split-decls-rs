// Generated macro for basic (function)
macro_rules! Depcrate_services_fs_serve_dir_testsbasic {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"basic"}
// Dependencies: {}
# [tokio :: test] async fn basic () { let svc = ServeDir :: new ("..") ; let req = Request :: builder () . uri ("/README.md") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; assert_eq ! (res . headers () ["content-type"] , "text/markdown") ; let body = body_into_text (res . into_body ()) . await ; let contents = std :: fs :: read_to_string ("../README.md") . unwrap () ; assert_eq ! (body , contents) ; }
};
}
