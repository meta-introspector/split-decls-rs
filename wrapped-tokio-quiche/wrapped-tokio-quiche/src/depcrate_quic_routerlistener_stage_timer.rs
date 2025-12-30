// Generated macro for listener_stage_timer (module)
macro_rules! Depcrate_quic_routerlistener_stage_timer {
() => {
// Module: crate::quic::router
// Provides: {"listener_stage_timer"}
// Dependencies: {}
# [cfg (feature = "perf-quic-listener-metrics")] mod listener_stage_timer { use foundations :: telemetry :: metrics :: TimeHistogram ; use std :: time :: Instant ; pub (super) struct ListenerStageTimer { start : Instant , time_hist : TimeHistogram , } impl ListenerStageTimer { pub (super) fn new (start : Instant , time_hist : TimeHistogram ,) -> ListenerStageTimer { ListenerStageTimer { start , time_hist } } } impl Drop for ListenerStageTimer { fn drop (& mut self) { self . time_hist . observe ((Instant :: now () - self . start) . as_nanos () as u64) ; } } }
};
}
