// Generated macro for method_not_allowed (function)
macro_rules! Depcrate_services_fs_serve_dir_testsmethod_not_allowed {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"method_not_allowed"}
// Dependencies: {}
# [tokio :: test] async fn method_not_allowed () { let svc = ServeDir :: new ("..") ; let req = Request :: builder () . method (Method :: POST) . uri ("/README.md") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: METHOD_NOT_ALLOWED) ; assert_eq ! (res . headers () [ALLOW] , "GET,HEAD") ; }
};
}
