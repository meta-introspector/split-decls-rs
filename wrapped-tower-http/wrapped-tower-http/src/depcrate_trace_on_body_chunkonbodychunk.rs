// Generated macro for OnBodyChunk (trait)
macro_rules! Depcrate_trace_on_body_chunkOnBodyChunk {
() => {
// Module: crate::trace::on_body_chunk
// Provides: {"OnBodyChunk"}
// Dependencies: {}
# [doc = " Trait used to tell [`Trace`] what to do when a body chunk has been sent."] # [doc = ""] # [doc = " See the [module docs](../trace/index.html#on_body_chunk) for details on exactly when the"] # [doc = " `on_body_chunk` callback is called."] # [doc = ""] # [doc = " [`Trace`]: super::Trace"] pub trait OnBodyChunk < B > { # [doc = " Do the thing."] # [doc = ""] # [doc = " `latency` is the duration since the response was sent or since the last body chunk as sent."] # [doc = ""] # [doc = " `span` is the `tracing` [`Span`], corresponding to this request, produced by the closure"] # [doc = " passed to [`TraceLayer::make_span_with`]. It can be used to [record field values][record]"] # [doc = " that weren't known when the span was created."] # [doc = ""] # [doc = " [`Span`]: https://docs.rs/tracing/latest/tracing/span/index.html"] # [doc = " [record]: https://docs.rs/tracing/latest/tracing/span/struct.Span.html#method.record"] # [doc = ""] # [doc = " If you're using [hyper] as your server `B` will most likely be [`Bytes`]."] # [doc = ""] # [doc = " [hyper]: https://hyper.rs"] # [doc = " [`Bytes`]: https://docs.rs/bytes/latest/bytes/struct.Bytes.html"] # [doc = " [`TraceLayer::make_span_with`]: crate::trace::TraceLayer::make_span_with"] fn on_body_chunk (& mut self , chunk : & B , latency : Duration , span : & Span) ; }
};
}
