// Generated macro for OnEos (trait)
macro_rules! Depcrate_trace_on_eosOnEos {
() => {
// Module: crate::trace::on_eos
// Provides: {"OnEos"}
// Dependencies: {}
# [doc = " Trait used to tell [`Trace`] what to do when a stream closes."] # [doc = ""] # [doc = " See the [module docs](../trace/index.html#on_eos) for details on exactly when the `on_eos`"] # [doc = " callback is called."] # [doc = ""] # [doc = " [`Trace`]: super::Trace"] pub trait OnEos { # [doc = " Do the thing."] # [doc = ""] # [doc = " `stream_duration` is the duration since the response was sent."] # [doc = ""] # [doc = " `span` is the `tracing` [`Span`], corresponding to this request, produced by the closure"] # [doc = " passed to [`TraceLayer::make_span_with`]. It can be used to [record field values][record]"] # [doc = " that weren't known when the span was created."] # [doc = ""] # [doc = " [`Span`]: https://docs.rs/tracing/latest/tracing/span/index.html"] # [doc = " [record]: https://docs.rs/tracing/latest/tracing/span/struct.Span.html#method.record"] # [doc = " [`TraceLayer::make_span_with`]: crate::trace::TraceLayer::make_span_with"] fn on_eos (self , trailers : Option < & HeaderMap > , stream_duration : Duration , span : & Span) ; }
};
}
