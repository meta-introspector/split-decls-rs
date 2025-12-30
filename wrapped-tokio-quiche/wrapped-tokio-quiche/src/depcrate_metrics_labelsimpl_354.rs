// Generated macro for impl_354 (impl)
macro_rules! Depcrate_metrics_labelsimpl_354 {
() => {
// Module: crate::metrics::labels
// Provides: {"impl_354"}
// Dependencies: {}
impl From < & quic :: HandshakeError > for HandshakeError { fn from (err : & quic :: HandshakeError) -> Self { match err { quic :: HandshakeError :: Timeout => Self :: Timeout , quic :: HandshakeError :: ConnectionClosed => Self :: Disconnect , } } }
};
}
