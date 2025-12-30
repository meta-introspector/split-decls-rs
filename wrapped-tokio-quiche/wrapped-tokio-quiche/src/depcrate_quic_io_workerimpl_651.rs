// Generated macro for impl_651 (impl)
macro_rules! Depcrate_quic_io_workerimpl_651 {
() => {
// Module: crate::quic::io::worker
// Provides: {"impl_651"}
// Dependencies: {}
impl < Tx , M , S > From < IoWorker < Tx , M , S > > for IoWorkerParams < Tx , M > { fn from (value : IoWorker < Tx , M , S >) -> Self { Self { socket : value . socket , shutdown_tx : value . shutdown_tx , cfg : value . cfg , audit_log_stats : value . audit_log_stats , write_state : value . write_state , conn_map_cmd_tx : value . conn_map_cmd_tx , # [cfg (feature = "perf-quic-listener-metrics")] init_rx_time : value . init_rx_time , metrics : value . metrics , } } }
};
}
