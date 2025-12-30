// Generated macro for impl_587 (impl)
macro_rules! Depcrate_quic_io_utilization_estimatorimpl_587 {
() => {
// Module: crate::quic::io::utilization_estimator
// Provides: {"impl_587"}
// Dependencies: {}
impl Drop for BandwidthReporter { fn drop (& mut self) { self . gauge . dec_by (self . last_bandwidth) ; } }
};
}
