// Generated macro for accept_encoding_identity (function)
macro_rules! Depcrate_services_fs_serve_dir_testsaccept_encoding_identity {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"accept_encoding_identity"}
// Dependencies: {}
# [tokio :: test] async fn accept_encoding_identity () { let svc = ServeDir :: new ("..") ; let req = Request :: builder () . uri ("/README.md") . header ("Accept-Encoding" , "identity") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; assert ! (res . headers () . get ("content-encoding") . is_none ()) ; }
};
}
