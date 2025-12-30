// Generated macro for WindowedFilter (struct)
macro_rules! Depcrate_quic_io_utilization_estimatorWindowedFilter {
() => {
// Module: crate::quic::io::utilization_estimator
// Provides: {"WindowedFilter"}
// Dependencies: {}
pub struct WindowedFilter < T , I , D > { window_length : D , estimates : [Option < Sample < T , I > > ; 3] , }
};
}
