// Generated macro for not_found (function)
macro_rules! Depcrate_services_fs_serve_dir_testsnot_found {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"not_found"}
// Dependencies: {}
# [tokio :: test] async fn not_found () { let svc = ServeDir :: new ("..") ; let req = Request :: builder () . uri ("/not-found") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: NOT_FOUND) ; assert ! (res . headers () . get (header :: CONTENT_TYPE) . is_none ()) ; let body = body_into_text (res . into_body ()) . await ; assert ! (body . is_empty ()) ; }
};
}
