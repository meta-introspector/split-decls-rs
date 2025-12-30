// Generated macro for impl_495 (impl)
macro_rules! Depcrate_trace_layerimpl_495 {
() => {
// Module: crate::trace::layer
// Provides: {"impl_495"}
// Dependencies: {}
impl TraceLayer < GrpcMakeClassifier > { # [doc = " Create a new [`TraceLayer`] using [`GrpcErrorsAsFailures`] which supports classifying"] # [doc = " gRPC responses and streams based on the `grpc-status` header."] pub fn new_for_grpc () -> Self { Self { make_classifier : SharedClassifier :: new (GrpcErrorsAsFailures :: default ()) , make_span : DefaultMakeSpan :: new () , on_response : DefaultOnResponse :: default () , on_request : DefaultOnRequest :: default () , on_body_chunk : DefaultOnBodyChunk :: default () , on_eos : DefaultOnEos :: default () , on_failure : DefaultOnFailure :: default () , } } }
};
}
