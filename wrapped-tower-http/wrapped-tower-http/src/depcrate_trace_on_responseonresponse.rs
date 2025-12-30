// Generated macro for OnResponse (trait)
macro_rules! Depcrate_trace_on_responseOnResponse {
() => {
// Module: crate::trace::on_response
// Provides: {"OnResponse"}
// Dependencies: {}
# [doc = " Trait used to tell [`Trace`] what to do when a response has been produced."] # [doc = ""] # [doc = " See the [module docs](../trace/index.html#on_response) for details on exactly when the"] # [doc = " `on_response` callback is called."] # [doc = ""] # [doc = " [`Trace`]: super::Trace"] pub trait OnResponse < B > { # [doc = " Do the thing."] # [doc = ""] # [doc = " `latency` is the duration since the request was received."] # [doc = ""] # [doc = " `span` is the `tracing` [`Span`], corresponding to this request, produced by the closure"] # [doc = " passed to [`TraceLayer::make_span_with`]. It can be used to [record field values][record]"] # [doc = " that weren't known when the span was created."] # [doc = ""] # [doc = " [`Span`]: https://docs.rs/tracing/latest/tracing/span/index.html"] # [doc = " [record]: https://docs.rs/tracing/latest/tracing/span/struct.Span.html#method.record"] # [doc = " [`TraceLayer::make_span_with`]: crate::trace::TraceLayer::make_span_with"] fn on_response (self , response : & Response < B > , latency : Duration , span : & Span) ; }
};
}
