// Generated macro for impl_580 (impl)
macro_rules! Depcrate_trace_serviceimpl_580 {
() => {
// Module: crate::trace::service
// Provides: {"impl_580"}
// Dependencies: {}
impl < S > Trace < S , GrpcMakeClassifier , DefaultMakeSpan , DefaultOnRequest , DefaultOnResponse , DefaultOnBodyChunk , DefaultOnEos , DefaultOnFailure , > { # [doc = " Create a new [`Trace`] using [`GrpcErrorsAsFailures`] which supports classifying"] # [doc = " gRPC responses and streams based on the `grpc-status` header."] pub fn new_for_grpc (inner : S) -> Self { Self { inner , make_classifier : SharedClassifier :: new (GrpcErrorsAsFailures :: default ()) , make_span : DefaultMakeSpan :: new () , on_request : DefaultOnRequest :: default () , on_response : DefaultOnResponse :: default () , on_body_chunk : DefaultOnBodyChunk :: default () , on_eos : DefaultOnEos :: default () , on_failure : DefaultOnFailure :: default () , } } }
};
}
