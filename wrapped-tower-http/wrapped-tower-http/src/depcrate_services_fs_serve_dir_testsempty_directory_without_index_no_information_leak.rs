// Generated macro for empty_directory_without_index_no_information_leak (function)
macro_rules! Depcrate_services_fs_serve_dir_testsempty_directory_without_index_no_information_leak {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"empty_directory_without_index_no_information_leak"}
// Dependencies: {}
# [tokio :: test] async fn empty_directory_without_index_no_information_leak () { let svc = ServeDir :: new ("..") . append_index_html_on_directories (false) ; let req = Request :: builder () . uri ("/test-files") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: NOT_FOUND) ; assert ! (res . headers () . get (header :: CONTENT_TYPE) . is_none ()) ; let body = body_into_text (res . into_body ()) . await ; assert ! (body . is_empty ()) ; }
};
}
