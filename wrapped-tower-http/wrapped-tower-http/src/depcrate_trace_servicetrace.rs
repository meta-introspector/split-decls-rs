// Generated macro for Trace (struct)
macro_rules! Depcrate_trace_serviceTrace {
() => {
// Module: crate::trace::service
// Provides: {"Trace"}
// Dependencies: {}
# [doc = " Middleware that adds high level [tracing] to a [`Service`]."] # [doc = ""] # [doc = " See the [module docs](crate::trace) for an example."] # [doc = ""] # [doc = " [tracing]: https://crates.io/crates/tracing"] # [doc = " [`Service`]: tower_service::Service"] # [derive (Debug , Clone , Copy)] pub struct Trace < S , M , MakeSpan = DefaultMakeSpan , OnRequest = DefaultOnRequest , OnResponse = DefaultOnResponse , OnBodyChunk = DefaultOnBodyChunk , OnEos = DefaultOnEos , OnFailure = DefaultOnFailure , > { pub (crate) inner : S , pub (crate) make_classifier : M , pub (crate) make_span : MakeSpan , pub (crate) on_request : OnRequest , pub (crate) on_response : OnResponse , pub (crate) on_body_chunk : OnBodyChunk , pub (crate) on_eos : OnEos , pub (crate) on_failure : OnFailure , }
};
}
