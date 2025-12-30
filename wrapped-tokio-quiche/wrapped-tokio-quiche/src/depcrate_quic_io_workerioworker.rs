// Generated macro for IoWorker (struct)
macro_rules! Depcrate_quic_io_workerIoWorker {
() => {
// Module: crate::quic::io::worker
// Provides: {"IoWorker"}
// Dependencies: {}
pub (crate) struct IoWorker < Tx , M , S > { socket : MaybeConnectedSocket < Tx > , # [doc = " A field that signals to the listener task that the connection has gone"] # [doc = " away (nothing is sent here, listener task just detects the sender"] # [doc = " has dropped)"] shutdown_tx : mpsc :: Sender < () > , cfg : WriterConfig , audit_log_stats : Arc < QuicAuditStats > , write_state : WriteState , conn_map_cmd_tx : mpsc :: UnboundedSender < ConnectionMapCommand > , # [cfg (feature = "perf-quic-listener-metrics")] init_rx_time : Option < SystemTime > , metrics : M , conn_stage : S , bw_estimator : BandwidthReporter , }
};
}
