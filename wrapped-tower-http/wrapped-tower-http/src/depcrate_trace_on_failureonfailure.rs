// Generated macro for OnFailure (trait)
macro_rules! Depcrate_trace_on_failureOnFailure {
() => {
// Module: crate::trace::on_failure
// Provides: {"OnFailure"}
// Dependencies: {}
# [doc = " Trait used to tell [`Trace`] what to do when a request fails."] # [doc = ""] # [doc = " See the [module docs](../trace/index.html#on_failure) for details on exactly when the"] # [doc = " `on_failure` callback is called."] # [doc = ""] # [doc = " [`Trace`]: super::Trace"] pub trait OnFailure < FailureClass > { # [doc = " Do the thing."] # [doc = ""] # [doc = " `latency` is the duration since the request was received."] # [doc = ""] # [doc = " `span` is the `tracing` [`Span`], corresponding to this request, produced by the closure"] # [doc = " passed to [`TraceLayer::make_span_with`]. It can be used to [record field values][record]"] # [doc = " that weren't known when the span was created."] # [doc = ""] # [doc = " [`Span`]: https://docs.rs/tracing/latest/tracing/span/index.html"] # [doc = " [record]: https://docs.rs/tracing/latest/tracing/span/struct.Span.html#method.record"] # [doc = " [`TraceLayer::make_span_with`]: crate::trace::TraceLayer::make_span_with"] fn on_failure (& mut self , failure_classification : FailureClass , latency : Duration , span : & Span) ; }
};
}
