// Generated macro for impl_935 (impl)
macro_rules! Depcrate_catch_panicimpl_935 {
() => {
// Module: crate::catch_panic
// Provides: {"impl_935"}
// Dependencies: {}
impl ResponseForPanic for DefaultResponseForPanic { type ResponseBody = Full ; fn response_for_panic (& mut self , err : Box < dyn Any + Send + 'static > ,) -> Response < Self :: ResponseBody > { if let Some (s) = err . downcast_ref :: < String > () { tracing :: error ! ("Service panicked: {}" , s) ; } else if let Some (s) = err . downcast_ref :: < & str > () { tracing :: error ! ("Service panicked: {}" , s) ; } else { tracing :: error ! ("Service panicked but `CatchPanic` was unable to downcast the panic info") ; } ; let mut res = Response :: new (Full :: new (http_body_util :: Full :: from ("Service panicked"))) ; * res . status_mut () = StatusCode :: INTERNAL_SERVER_ERROR ; # [allow (clippy :: declare_interior_mutable_const)] const TEXT_PLAIN : HeaderValue = HeaderValue :: from_static ("text/plain; charset=utf-8") ; res . headers_mut () . insert (http :: header :: CONTENT_TYPE , TEXT_PLAIN) ; res } }
};
}
