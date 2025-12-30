// Generated macro for basic_with_index (function)
macro_rules! Depcrate_services_fs_serve_dir_testsbasic_with_index {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"basic_with_index"}
// Dependencies: {}
# [tokio :: test] async fn basic_with_index () { let svc = ServeDir :: new ("../test-files") ; let req = Request :: new (Body :: empty ()) ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; assert_eq ! (res . headers () [header :: CONTENT_TYPE] , "text/html") ; let body = body_into_text (res . into_body ()) . await ; assert_eq ! (body , "<b>HTML!</b>\n") ; }
};
}
