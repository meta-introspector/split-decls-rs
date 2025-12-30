// Generated macro for IoWorkerParams (struct)
macro_rules! Depcrate_quic_io_workerIoWorkerParams {
() => {
// Module: crate::quic::io::worker
// Provides: {"IoWorkerParams"}
// Dependencies: {}
pub (crate) struct IoWorkerParams < Tx , M > { pub (crate) socket : MaybeConnectedSocket < Tx > , pub (crate) shutdown_tx : mpsc :: Sender < () > , pub (crate) cfg : WriterConfig , pub (crate) audit_log_stats : Arc < QuicAuditStats > , pub (crate) write_state : WriteState , pub (crate) conn_map_cmd_tx : mpsc :: UnboundedSender < ConnectionMapCommand > , # [cfg (feature = "perf-quic-listener-metrics")] pub (crate) init_rx_time : Option < SystemTime > , pub (crate) metrics : M , }
};
}
