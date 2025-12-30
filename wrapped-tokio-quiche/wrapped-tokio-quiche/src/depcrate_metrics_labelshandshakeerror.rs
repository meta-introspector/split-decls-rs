// Generated macro for HandshakeError (enum)
macro_rules! Depcrate_metrics_labelsHandshakeError {
() => {
// Module: crate::metrics::labels
// Provides: {"HandshakeError"}
// Dependencies: {}
# [doc = " Category of error that caused the QUIC handshake to fail."] # [derive (Clone , Eq , Hash , PartialEq , Serialize)] # [serde (rename_all = "lowercase")] pub enum HandshakeError { CryptoFail , TlsFail , Timeout , Disconnect , Other , }
};
}
