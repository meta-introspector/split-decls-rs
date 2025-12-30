// Generated macro for call_fallback (function)
macro_rules! Depcrate_services_fs_serve_dir_futurecall_fallback {
() => {
// Module: crate::services::fs::serve_dir::future
// Provides: {"call_fallback"}
// Dependencies: {}
pub (super) fn call_fallback < F , B , FResBody > (fallback : & mut F , req : Request < B > ,) -> ResponseFutureInner < B , F > where F : Service < Request < B > , Response = Response < FResBody > , Error = Infallible > + Clone , F :: Future : Send + 'static , FResBody : http_body :: Body < Data = Bytes > + Send + 'static , FResBody :: Error : Into < BoxError > , { let future = fallback . call (req) . map_ok (| response | { response . map (| body | { UnsyncBoxBody :: new (body . map_err (| err | match err . into () . downcast :: < io :: Error > () { Ok (err) => * err , Err (err) => io :: Error :: new (io :: ErrorKind :: Other , err) , }) . boxed_unsync () ,) }) . map (ResponseBody :: new) }) . boxed () ; ResponseFutureInner :: FallbackFuture { future } }
};
}
