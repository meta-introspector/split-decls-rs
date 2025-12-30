// Generated macro for impl_566 (impl)
macro_rules! Depcrate_trace_on_responseimpl_566 {
() => {
// Module: crate::trace::on_response
// Provides: {"impl_566"}
// Dependencies: {}
impl DefaultOnResponse { # [doc = " Create a new `DefaultOnResponse`."] pub fn new () -> Self { Self :: default () } # [doc = " Set the [`Level`] used for [tracing events]."] # [doc = ""] # [doc = " Please note that while this will set the level for the tracing events"] # [doc = " themselves, it might cause them to lack expected information, like"] # [doc = " request method or path. You can address this using"] # [doc = " [`DefaultMakeSpan::level`]."] # [doc = ""] # [doc = " Defaults to [`Level::DEBUG`]."] # [doc = ""] # [doc = " [tracing events]: https://docs.rs/tracing/latest/tracing/#events"] # [doc = " [`DefaultMakeSpan::level`]: crate::trace::DefaultMakeSpan::level"] pub fn level (mut self , level : Level) -> Self { self . level = level ; self } # [doc = " Set the [`LatencyUnit`] latencies will be reported in."] # [doc = ""] # [doc = " Defaults to [`LatencyUnit::Millis`]."] pub fn latency_unit (mut self , latency_unit : LatencyUnit) -> Self { self . latency_unit = latency_unit ; self } # [doc = " Include response headers on the [`Event`]."] # [doc = ""] # [doc = " By default headers are not included."] # [doc = ""] # [doc = " [`Event`]: tracing::Event"] pub fn include_headers (mut self , include_headers : bool) -> Self { self . include_headers = include_headers ; self } }
};
}
