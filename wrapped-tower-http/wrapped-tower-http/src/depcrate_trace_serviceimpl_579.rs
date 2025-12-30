// Generated macro for impl_579 (impl)
macro_rules! Depcrate_trace_serviceimpl_579 {
() => {
// Module: crate::trace::service
// Provides: {"impl_579"}
// Dependencies: {}
impl < S > Trace < S , HttpMakeClassifier , DefaultMakeSpan , DefaultOnRequest , DefaultOnResponse , DefaultOnBodyChunk , DefaultOnEos , DefaultOnFailure , > { # [doc = " Create a new [`Trace`] using [`ServerErrorsAsFailures`] which supports classifying"] # [doc = " regular HTTP responses based on the status code."] pub fn new_for_http (inner : S) -> Self { Self { inner , make_classifier : SharedClassifier :: new (ServerErrorsAsFailures :: default ()) , make_span : DefaultMakeSpan :: new () , on_request : DefaultOnRequest :: default () , on_response : DefaultOnResponse :: default () , on_body_chunk : DefaultOnBodyChunk :: default () , on_eos : DefaultOnEos :: default () , on_failure : DefaultOnFailure :: default () , } } }
};
}
