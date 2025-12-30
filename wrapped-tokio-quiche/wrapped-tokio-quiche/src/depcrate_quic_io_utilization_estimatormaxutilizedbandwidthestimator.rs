// Generated macro for MaxUtilizedBandwidthEstimator (struct)
macro_rules! Depcrate_quic_io_utilization_estimatorMaxUtilizedBandwidthEstimator {
() => {
// Module: crate::quic::io::utilization_estimator
// Provides: {"MaxUtilizedBandwidthEstimator"}
// Dependencies: {}
pub (super) struct MaxUtilizedBandwidthEstimator { rounds : VecDeque < Round > , estimate : WindowedFilter < Estimate , Instant , Duration > , bytes_sent_prev_round : u64 , }
};
}
