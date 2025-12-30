// Generated macro for impl_353 (impl)
macro_rules! Depcrate_metrics_labelsimpl_353 {
() => {
// Module: crate::metrics::labels
// Provides: {"impl_353"}
// Dependencies: {}
impl From < & quiche :: Error > for HandshakeError { fn from (err : & quiche :: Error) -> Self { match err { quiche :: Error :: CryptoFail => Self :: CryptoFail , quiche :: Error :: TlsFail => Self :: TlsFail , _ => Self :: Other , } } }
};
}
