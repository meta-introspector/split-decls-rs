// Generated macro for ServerHelloPayload (struct)
macro_rules! Depcrate_msgs_handshakeServerHelloPayload {
() => {
// Module: crate::msgs::handshake
// Provides: {"ServerHelloPayload"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) struct ServerHelloPayload { pub (crate) legacy_version : ProtocolVersion , pub (crate) random : Random , pub (crate) session_id : SessionId , pub (crate) cipher_suite : CipherSuite , pub (crate) compression_method : Compression , pub (crate) extensions : Box < ServerExtensions < 'static > > , }
};
}
