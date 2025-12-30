// Generated macro for macro_485 (macro)
macro_rules! Depcrate_trace_futuremacro_485 {
() => {
// Module: crate::trace::future
// Provides: {"macro_485"}
// Dependencies: {}
pin_project ! { # [doc = " Response future for [`Trace`]."] # [doc = ""] # [doc = " [`Trace`]: super::Trace"] pub struct ResponseFuture < F , C , OnResponse = DefaultOnResponse , OnBodyChunk = DefaultOnBodyChunk , OnEos = DefaultOnEos , OnFailure = DefaultOnFailure > { # [pin] pub (crate) inner : F , pub (crate) span : Span , pub (crate) classifier : Option < C >, pub (crate) on_response : Option < OnResponse >, pub (crate) on_body_chunk : Option < OnBodyChunk >, pub (crate) on_eos : Option < OnEos >, pub (crate) on_failure : Option < OnFailure >, pub (crate) start : Instant , } }
};
}
