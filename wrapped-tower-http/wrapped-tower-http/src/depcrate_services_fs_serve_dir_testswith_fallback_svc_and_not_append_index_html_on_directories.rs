// Generated macro for with_fallback_svc_and_not_append_index_html_on_directories (function)
macro_rules! Depcrate_services_fs_serve_dir_testswith_fallback_svc_and_not_append_index_html_on_directories {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"with_fallback_svc_and_not_append_index_html_on_directories"}
// Dependencies: {}
# [tokio :: test] async fn with_fallback_svc_and_not_append_index_html_on_directories () { async fn fallback < B > (req : Request < B >) -> Result < Response < Body > , Infallible > { Ok (Response :: new (Body :: from (format ! ("from fallback {}" , req . uri () . path ())))) } let svc = ServeDir :: new ("..") . append_index_html_on_directories (false) . fallback (tower :: service_fn (fallback)) ; let req = Request :: builder () . uri ("/") . body (Body :: empty ()) . unwrap () ; let res = svc . oneshot (req) . await . unwrap () ; assert_eq ! (res . status () , StatusCode :: OK) ; let body = body_into_text (res . into_body ()) . await ; assert_eq ! (body , "from fallback /") ; }
};
}
