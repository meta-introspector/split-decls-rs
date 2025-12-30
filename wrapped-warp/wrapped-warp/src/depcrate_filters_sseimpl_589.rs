// Generated macro for impl_589 (impl)
macro_rules! Depcrate_filters_sseimpl_589 {
() => {
// Module: crate::filters::sse
// Provides: {"impl_589"}
// Dependencies: {}
impl < S > Reply for SseReply < S > where S : TryStream < Ok = Event > + Send + Sync + 'static , S :: Error : StdError + Send + Sync + 'static , { # [inline] fn into_response (self) -> Response { let body_stream = self . event_stream . map_err (| error | { log :: error ! ("sse stream error: {}" , error) ; SseError }) . into_stream () . and_then (| event | future :: ready (Ok (event . to_string ()))) ; let mut res = Response :: new (Body :: wrap_stream (body_stream)) ; res . headers_mut () . insert (CONTENT_TYPE , HeaderValue :: from_static ("text/event-stream")) ; res . headers_mut () . insert (CACHE_CONTROL , HeaderValue :: from_static ("no-cache")) ; res } }
};
}
