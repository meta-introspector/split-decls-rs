// Generated macro for impl_528 (impl)
macro_rules! Depcrate_trace_on_eosimpl_528 {
() => {
// Module: crate::trace::on_eos
// Provides: {"impl_528"}
// Dependencies: {}
impl DefaultOnEos { # [doc = " Create a new [`DefaultOnEos`]."] pub fn new () -> Self { Self :: default () } # [doc = " Set the [`Level`] used for [tracing events]."] # [doc = ""] # [doc = " Defaults to [`Level::DEBUG`]."] # [doc = ""] # [doc = " [tracing events]: https://docs.rs/tracing/latest/tracing/#events"] # [doc = " [`Level::DEBUG`]: https://docs.rs/tracing/latest/tracing/struct.Level.html#associatedconstant.DEBUG"] pub fn level (mut self , level : Level) -> Self { self . level = level ; self } # [doc = " Set the [`LatencyUnit`] latencies will be reported in."] # [doc = ""] # [doc = " Defaults to [`LatencyUnit::Millis`]."] pub fn latency_unit (mut self , latency_unit : LatencyUnit) -> Self { self . latency_unit = latency_unit ; self } }
};
}
