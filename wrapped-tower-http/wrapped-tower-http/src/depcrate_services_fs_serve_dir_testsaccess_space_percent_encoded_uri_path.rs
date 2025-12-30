// Generated macro for access_space_percent_encoded_uri_path (function)
macro_rules! Depcrate_services_fs_serve_dir_testsaccess_space_percent_encoded_uri_path {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"access_space_percent_encoded_uri_path"}
// Dependencies: {}
# [tokio :: test] async fn access_space_percent_encoded_uri_path () { let encoded_filename = "filename%20with%20space.txt" ; let svc = ServeDir :: new ("../test-files") ; let req = Request :: builder () . uri (format ! ("/{}" , encoded_filename)) . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; assert_eq ! (res . headers () ["content-type"] , "text/plain") ; }
};
}
