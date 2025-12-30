// Generated macro for calling_fallback_on_not_allowed (function)
macro_rules! Depcrate_services_fs_serve_dir_testscalling_fallback_on_not_allowed {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"calling_fallback_on_not_allowed"}
// Dependencies: {}
# [tokio :: test] async fn calling_fallback_on_not_allowed () { async fn fallback < B > (req : Request < B >) -> Result < Response < Body > , Infallible > { Ok (Response :: new (Body :: from (format ! ("from fallback {}" , req . uri () . path ())))) } let svc = ServeDir :: new ("..") . call_fallback_on_method_not_allowed (true) . fallback (tower :: service_fn (fallback)) ; let req = Request :: builder () . method (Method :: POST) . uri ("/doesnt-exist") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; let body = body_into_text (res . into_body ()) . await ; assert_eq ! (body , "from fallback /doesnt-exist") ; }
};
}
