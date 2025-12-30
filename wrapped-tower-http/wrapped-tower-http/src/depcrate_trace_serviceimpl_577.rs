// Generated macro for impl_577 (impl)
macro_rules! Depcrate_trace_serviceimpl_577 {
() => {
// Module: crate::trace::service
// Provides: {"impl_577"}
// Dependencies: {}
impl < S , M > Trace < S , M > { # [doc = " Create a new [`Trace`] using the given [`MakeClassifier`]."] pub fn new (inner : S , make_classifier : M) -> Self where M : MakeClassifier , { Self { inner , make_classifier , make_span : DefaultMakeSpan :: new () , on_request : DefaultOnRequest :: default () , on_response : DefaultOnResponse :: default () , on_body_chunk : DefaultOnBodyChunk :: default () , on_eos : DefaultOnEos :: default () , on_failure : DefaultOnFailure :: default () , } } # [doc = " Returns a new [`Layer`] that wraps services with a [`TraceLayer`] middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer (make_classifier : M) -> TraceLayer < M > where M : MakeClassifier , { TraceLayer :: new (make_classifier) } }
};
}
