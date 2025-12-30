// Generated macro for with_fallback_serve_file (function)
macro_rules! Depcrate_services_fs_serve_dir_testswith_fallback_serve_file {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"with_fallback_serve_file"}
// Dependencies: {}
# [tokio :: test] async fn with_fallback_serve_file () { let svc = ServeDir :: new ("..") . fallback (ServeFile :: new ("../README.md")) ; let req = Request :: builder () . uri ("/doesnt-exist") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; assert_eq ! (res . headers () ["content-type"] , "text/markdown") ; let body = body_into_text (res . into_body ()) . await ; let contents = std :: fs :: read_to_string ("../README.md") . unwrap () ; assert_eq ! (body , contents) ; }
};
}
