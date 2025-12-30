// Generated macro for impl_586 (impl)
macro_rules! Depcrate_quic_io_utilization_estimatorimpl_586 {
() => {
// Module: crate::quic::io::utilization_estimator
// Provides: {"impl_586"}
// Dependencies: {}
impl BandwidthReporter { pub (super) fn new (gauge : Gauge) -> Self { BandwidthReporter { last_update : Instant :: now () , update_period : Duration :: from_millis (50) , last_bandwidth : 0 , last_sent : 0 , last_lost : 0 , last_acked : 0 , max_bandwidth : 0 , max_loss_pct : 0. , estimator : MaxUtilizedBandwidthEstimator :: new () , gauge , } } # [inline] pub (super) fn update (& mut self , quiche : & QuicheConnection , now : Instant) { if now . duration_since (self . last_update) < self . update_period { return ; } let stats = quiche . stats () ; let bytes_sent = stats . sent_bytes - self . last_sent ; let bytes_lost = stats . lost_bytes - self . last_lost ; let bytes_acked = stats . acked_bytes - self . last_acked ; self . estimator . new_round (self . last_update , bytes_sent , bytes_lost , bytes_acked ,) ; self . last_sent = stats . sent_bytes ; self . last_lost = stats . lost_bytes ; self . last_acked = stats . acked_bytes ; self . last_update = now ; let bw_estimate = self . estimator . get () ; if self . last_bandwidth != bw_estimate . bandwidth { self . gauge . dec_by (self . last_bandwidth) ; self . last_bandwidth = bw_estimate . bandwidth ; self . gauge . inc_by (self . last_bandwidth) ; self . max_bandwidth = self . max_bandwidth . max (self . last_bandwidth) ; self . max_loss_pct = self . max_loss_pct . max (bw_estimate . loss) ; } if let Some (p) = quiche . path_stats () . find (| s | s . active) { self . update_period = p . rtt ; } } }
};
}
