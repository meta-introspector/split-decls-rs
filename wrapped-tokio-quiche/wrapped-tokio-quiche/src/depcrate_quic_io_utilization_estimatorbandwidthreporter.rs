// Generated macro for BandwidthReporter (struct)
macro_rules! Depcrate_quic_io_utilization_estimatorBandwidthReporter {
() => {
// Module: crate::quic::io::utilization_estimator
// Provides: {"BandwidthReporter"}
// Dependencies: {}
# [doc = " [`BandwidthReporter`] is responsible to track the bandwidth estimate for the"] # [doc = " connection"] pub (super) struct BandwidthReporter { # [doc = " Time of last update"] last_update : Instant , # [doc = " Period between update (set using rtt)"] update_period : Duration , # [doc = " Estimate at last update"] last_bandwidth : u64 , # [doc = " Bytes sent at last update"] last_sent : u64 , # [doc = " Bytes lost at last update"] last_lost : u64 , # [doc = " Bytes acked at last update"] last_acked : u64 , # [doc = " Max recorded bandwidth"] pub (super) max_bandwidth : u64 , # [doc = " Loss at max recorded bandwidth"] pub (super) max_loss_pct : f32 , estimator : MaxUtilizedBandwidthEstimator , gauge : Gauge , }
};
}
