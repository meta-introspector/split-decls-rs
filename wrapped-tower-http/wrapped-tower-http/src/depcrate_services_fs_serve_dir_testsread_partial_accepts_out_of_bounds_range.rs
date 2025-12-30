// Generated macro for read_partial_accepts_out_of_bounds_range (function)
macro_rules! Depcrate_services_fs_serve_dir_testsread_partial_accepts_out_of_bounds_range {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"read_partial_accepts_out_of_bounds_range"}
// Dependencies: {}
# [tokio :: test] async fn read_partial_accepts_out_of_bounds_range () { let svc = ServeDir :: new ("..") ; let bytes_start_incl = 0 ; let bytes_end_excl = 9999999 ; let requested_len = bytes_end_excl - bytes_start_incl ; let req = Request :: builder () . uri ("/README.md") . header ("Range" , format ! ("bytes={}-{}" , bytes_start_incl , requested_len - 1) ,) . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: PARTIAL_CONTENT) ; let file_contents = std :: fs :: read ("../README.md") . unwrap () ; assert_eq ! (res . headers () ["content-range"] , & format ! ("bytes 0-{}/{}" , file_contents . len () - 1 , file_contents . len ())) }
};
}
