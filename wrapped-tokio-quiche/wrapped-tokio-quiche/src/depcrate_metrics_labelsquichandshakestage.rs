// Generated macro for QuicHandshakeStage (enum)
macro_rules! Depcrate_metrics_labelsQuicHandshakeStage {
() => {
// Module: crate::metrics::labels
// Provides: {"QuicHandshakeStage"}
// Dependencies: {}
# [doc = " Type of handshake latency that was measured by a metric."] # [derive (Clone , Eq , Hash , PartialEq , Serialize)] # [serde (rename_all = "lowercase")] pub enum QuicHandshakeStage { QueueWaiting , HandshakeProtocol , HandshakeResponse , }
};
}
