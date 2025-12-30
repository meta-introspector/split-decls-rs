// Generated macro for ClientBuilder (struct)
macro_rules! Depcrate_secure_transportClientBuilder {
() => {
// Module: crate::secure_transport
// Provides: {"ClientBuilder"}
// Dependencies: {}
# [doc = " A builder type to simplify the creation of client side `SslStream`s."] # [derive (Debug)] pub struct ClientBuilder { identity : Option < SecIdentity > , certs : Vec < SecCertificate > , chain : Vec < SecCertificate > , protocol_min : Option < SslProtocol > , protocol_max : Option < SslProtocol > , trust_certs_only : bool , use_sni : bool , danger_accept_invalid_certs : bool , danger_accept_invalid_hostnames : bool , whitelisted_ciphers : Vec < CipherSuite > , blacklisted_ciphers : Vec < CipherSuite > , # [cfg (feature = "alpn")] alpn : Option < Vec < String > > , # [cfg (feature = "session-tickets")] enable_session_tickets : bool , }
};
}
