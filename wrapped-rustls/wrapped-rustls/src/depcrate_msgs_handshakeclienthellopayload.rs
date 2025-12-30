// Generated macro for ClientHelloPayload (struct)
macro_rules! Depcrate_msgs_handshakeClientHelloPayload {
() => {
// Module: crate::msgs::handshake
// Provides: {"ClientHelloPayload"}
// Dependencies: {}
# [derive (Clone , Debug)] pub (crate) struct ClientHelloPayload { pub (crate) client_version : ProtocolVersion , pub (crate) random : Random , pub (crate) session_id : SessionId , pub (crate) cipher_suites : Vec < CipherSuite > , pub (crate) compression_methods : Vec < Compression > , pub (crate) extensions : Box < ClientExtensions < 'static > > , }
};
}
