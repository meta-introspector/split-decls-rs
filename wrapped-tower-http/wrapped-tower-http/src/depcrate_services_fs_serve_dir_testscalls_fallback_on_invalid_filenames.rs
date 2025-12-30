// Generated macro for calls_fallback_on_invalid_filenames (function)
macro_rules! Depcrate_services_fs_serve_dir_testscalls_fallback_on_invalid_filenames {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"calls_fallback_on_invalid_filenames"}
// Dependencies: {}
# [tokio :: test] async fn calls_fallback_on_invalid_filenames () { async fn fallback < T > (_ : T) -> Result < Response < Body > , Infallible > { let mut res = Response :: new (Body :: empty ()) ; res . headers_mut () . insert ("from-fallback" , "1" . parse () . unwrap ()) ; Ok (res) } let svc = ServeDir :: new ("..") . fallback (service_fn (fallback)) ; let req = Request :: builder () . uri ("/invalid|path") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . headers () ["from-fallback"] , "1") ; }
};
}
