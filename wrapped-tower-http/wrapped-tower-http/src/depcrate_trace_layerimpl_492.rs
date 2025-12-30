// Generated macro for impl_492 (impl)
macro_rules! Depcrate_trace_layerimpl_492 {
() => {
// Module: crate::trace::layer
// Provides: {"impl_492"}
// Dependencies: {}
impl < M > TraceLayer < M > { # [doc = " Create a new [`TraceLayer`] using the given [`MakeClassifier`]."] pub fn new (make_classifier : M) -> Self where M : MakeClassifier , { Self { make_classifier , make_span : DefaultMakeSpan :: new () , on_failure : DefaultOnFailure :: default () , on_request : DefaultOnRequest :: default () , on_eos : DefaultOnEos :: default () , on_body_chunk : DefaultOnBodyChunk :: default () , on_response : DefaultOnResponse :: default () , } } }
};
}
