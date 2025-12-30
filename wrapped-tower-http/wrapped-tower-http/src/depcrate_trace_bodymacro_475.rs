// Generated macro for macro_475 (macro)
macro_rules! Depcrate_trace_bodymacro_475 {
() => {
// Module: crate::trace::body
// Provides: {"macro_475"}
// Dependencies: {}
pin_project ! { # [doc = " Response body for [`Trace`]."] # [doc = ""] # [doc = " [`Trace`]: super::Trace"] pub struct ResponseBody < B , C , OnBodyChunk = DefaultOnBodyChunk , OnEos = DefaultOnEos , OnFailure = DefaultOnFailure > { # [pin] pub (crate) inner : B , pub (crate) classify_eos : Option < C >, pub (crate) on_eos : Option < (OnEos , Instant) >, pub (crate) on_body_chunk : OnBodyChunk , pub (crate) on_failure : Option < OnFailure >, pub (crate) start : Instant , pub (crate) span : Span , } }
};
}
