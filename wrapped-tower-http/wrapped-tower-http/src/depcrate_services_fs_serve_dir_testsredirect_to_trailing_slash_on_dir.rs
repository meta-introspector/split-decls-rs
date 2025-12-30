// Generated macro for redirect_to_trailing_slash_on_dir (function)
macro_rules! Depcrate_services_fs_serve_dir_testsredirect_to_trailing_slash_on_dir {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"redirect_to_trailing_slash_on_dir"}
// Dependencies: {}
# [tokio :: test] async fn redirect_to_trailing_slash_on_dir () { let svc = ServeDir :: new (".") ; let req = Request :: builder () . uri ("/src") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: TEMPORARY_REDIRECT) ; let location = & res . headers () [http :: header :: LOCATION] ; assert_eq ! (location , "/src/") ; }
};
}
