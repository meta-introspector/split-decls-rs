// Generated macro for MakeSpan (trait)
macro_rules! Depcrate_trace_make_spanMakeSpan {
() => {
// Module: crate::trace::make_span
// Provides: {"MakeSpan"}
// Dependencies: {}
# [doc = " Trait used to generate [`Span`]s from requests. [`Trace`] wraps all request handling in this"] # [doc = " span."] # [doc = ""] # [doc = " [`Span`]: tracing::Span"] # [doc = " [`Trace`]: super::Trace"] pub trait MakeSpan < B > { # [doc = " Make a span from a request."] fn make_span (& mut self , request : & Request < B >) -> Span ; }
};
}
