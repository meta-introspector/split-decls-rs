// Generated macro for impl_494 (impl)
macro_rules! Depcrate_trace_layerimpl_494 {
() => {
// Module: crate::trace::layer
// Provides: {"impl_494"}
// Dependencies: {}
impl TraceLayer < HttpMakeClassifier > { # [doc = " Create a new [`TraceLayer`] using [`ServerErrorsAsFailures`] which supports classifying"] # [doc = " regular HTTP responses based on the status code."] pub fn new_for_http () -> Self { Self { make_classifier : SharedClassifier :: new (ServerErrorsAsFailures :: default ()) , make_span : DefaultMakeSpan :: new () , on_response : DefaultOnResponse :: default () , on_request : DefaultOnRequest :: default () , on_body_chunk : DefaultOnBodyChunk :: default () , on_eos : DefaultOnEos :: default () , on_failure : DefaultOnFailure :: default () , } } }
};
}
