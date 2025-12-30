// Generated macro for TraceLayer (struct)
macro_rules! Depcrate_trace_layerTraceLayer {
() => {
// Module: crate::trace::layer
// Provides: {"TraceLayer"}
// Dependencies: {}
# [doc = " [`Layer`] that adds high level [tracing] to a [`Service`]."] # [doc = ""] # [doc = " See the [module docs](crate::trace) for more details."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] # [doc = " [tracing]: https://crates.io/crates/tracing"] # [doc = " [`Service`]: tower_service::Service"] # [derive (Debug , Copy , Clone)] pub struct TraceLayer < M , MakeSpan = DefaultMakeSpan , OnRequest = DefaultOnRequest , OnResponse = DefaultOnResponse , OnBodyChunk = DefaultOnBodyChunk , OnEos = DefaultOnEos , OnFailure = DefaultOnFailure , > { pub (crate) make_classifier : M , pub (crate) make_span : MakeSpan , pub (crate) on_request : OnRequest , pub (crate) on_response : OnResponse , pub (crate) on_body_chunk : OnBodyChunk , pub (crate) on_eos : OnEos , pub (crate) on_failure : OnFailure , }
};
}
