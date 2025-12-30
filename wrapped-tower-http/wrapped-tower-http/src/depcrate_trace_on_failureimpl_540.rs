// Generated macro for impl_540 (impl)
macro_rules! Depcrate_trace_on_failureimpl_540 {
() => {
// Module: crate::trace::on_failure
// Provides: {"impl_540"}
// Dependencies: {}
impl DefaultOnFailure { # [doc = " Create a new `DefaultOnFailure`."] pub fn new () -> Self { Self :: default () } # [doc = " Set the [`Level`] used for [tracing events]."] # [doc = ""] # [doc = " Defaults to [`Level::ERROR`]."] # [doc = ""] # [doc = " [tracing events]: https://docs.rs/tracing/latest/tracing/#events"] pub fn level (mut self , level : Level) -> Self { self . level = level ; self } # [doc = " Set the [`LatencyUnit`] latencies will be reported in."] # [doc = ""] # [doc = " Defaults to [`LatencyUnit::Millis`]."] pub fn latency_unit (mut self , latency_unit : LatencyUnit) -> Self { self . latency_unit = latency_unit ; self } }
};
}
