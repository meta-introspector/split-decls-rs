// Generated macro for HelloRetryRequest (struct)
macro_rules! Depcrate_msgs_handshakeHelloRetryRequest {
() => {
// Module: crate::msgs::handshake
// Provides: {"HelloRetryRequest"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) struct HelloRetryRequest { pub (crate) legacy_version : ProtocolVersion , pub (crate) session_id : SessionId , pub (crate) cipher_suite : CipherSuite , pub (crate) extensions : HelloRetryRequestExtensions < 'static > , }
};
}
