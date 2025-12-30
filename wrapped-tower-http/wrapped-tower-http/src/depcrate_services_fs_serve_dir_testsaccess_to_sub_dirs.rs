// Generated macro for access_to_sub_dirs (function)
macro_rules! Depcrate_services_fs_serve_dir_testsaccess_to_sub_dirs {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"access_to_sub_dirs"}
// Dependencies: {}
# [tokio :: test] async fn access_to_sub_dirs () { let svc = ServeDir :: new ("..") ; let req = Request :: builder () . uri ("/tower-http/Cargo.toml") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; assert_eq ! (res . headers () ["content-type"] , "text/x-toml") ; let body = body_into_text (res . into_body ()) . await ; let contents = std :: fs :: read_to_string ("Cargo.toml") . unwrap () ; assert_eq ! (body , contents) ; }
};
}
