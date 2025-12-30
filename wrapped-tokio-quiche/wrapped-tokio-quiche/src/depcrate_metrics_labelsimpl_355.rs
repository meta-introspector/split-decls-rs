// Generated macro for impl_355 (impl)
macro_rules! Depcrate_metrics_labelsimpl_355 {
() => {
// Module: crate::metrics::labels
// Provides: {"impl_355"}
// Dependencies: {}
impl From < & BoxError > for HandshakeError { fn from (err : & BoxError) -> Self { if let Some (e) = err . downcast_ref :: < quic :: HandshakeError > () { Self :: from (e) } else if let Some (e) = err . downcast_ref :: < quiche :: Error > () { Self :: from (e) } else { Self :: Other } } }
};
}
