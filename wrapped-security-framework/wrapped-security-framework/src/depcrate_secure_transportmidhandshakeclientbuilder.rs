// Generated macro for MidHandshakeClientBuilder (struct)
macro_rules! Depcrate_secure_transportMidHandshakeClientBuilder {
() => {
// Module: crate::secure_transport
// Provides: {"MidHandshakeClientBuilder"}
// Dependencies: {}
# [doc = " An SSL stream midway through the handshake process."] # [derive (Debug)] pub struct MidHandshakeClientBuilder < S > { stream : MidHandshakeSslStream < S > , domain : Option < String > , certs : Vec < SecCertificate > , trust_certs_only : bool , danger_accept_invalid_certs : bool , }
};
}
