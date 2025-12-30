// Generated macro for impl_300 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_300 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_300"}
// Dependencies: {}
impl HelloRetryRequestExtensions < '_ > { fn into_owned (self) -> HelloRetryRequestExtensions < 'static > { let Self { key_share , cookie , supported_versions , encrypted_client_hello , order , } = self ; HelloRetryRequestExtensions { key_share , cookie , supported_versions , encrypted_client_hello : encrypted_client_hello . map (| x | x . into_owned ()) , order , } } }
};
}
