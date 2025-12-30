// Generated macro for with_custom_chunk_size (function)
macro_rules! Depcrate_services_fs_serve_dir_testswith_custom_chunk_size {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"with_custom_chunk_size"}
// Dependencies: {}
# [tokio :: test] async fn with_custom_chunk_size () { let svc = ServeDir :: new ("..") . with_buf_chunk_size (1024 * 32) ; let req = Request :: builder () . uri ("/README.md") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; assert_eq ! (res . headers () ["content-type"] , "text/markdown") ; let body = body_into_text (res . into_body ()) . await ; let contents = std :: fs :: read_to_string ("../README.md") . unwrap () ; assert_eq ! (body , contents) ; }
};
}
