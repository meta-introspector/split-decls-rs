// Generated macro for read_partial_in_bounds (function)
macro_rules! Depcrate_services_fs_serve_dir_testsread_partial_in_bounds {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"read_partial_in_bounds"}
// Dependencies: {}
# [tokio :: test] async fn read_partial_in_bounds () { let svc = ServeDir :: new ("..") ; let bytes_start_incl = 9 ; let bytes_end_incl = 1023 ; let req = Request :: builder () . uri ("/README.md") . header ("Range" , format ! ("bytes={}-{}" , bytes_start_incl , bytes_end_incl) ,) . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; let file_contents = std :: fs :: read ("../README.md") . unwrap () ; assert_eq ! (res . status () , StatusCode :: PARTIAL_CONTENT) ; assert_eq ! (res . headers () ["content-length"] , (bytes_end_incl - bytes_start_incl + 1) . to_string ()) ; assert ! (res . headers () ["content-range"] . to_str () . unwrap () . starts_with (& format ! ("bytes {}-{}/{}" , bytes_start_incl , bytes_end_incl , file_contents . len ()))) ; assert_eq ! (res . headers () ["content-type"] , "text/markdown") ; let body = to_bytes (res . into_body ()) . await . ok () . unwrap () ; let source = Bytes :: from (file_contents [bytes_start_incl ..= bytes_end_incl] . to_vec ()) ; assert_eq ! (body , source) ; }
};
}
