// Generated macro for impl_1794 (impl)
macro_rules! Depcrate_tls13impl_1794 {
() => {
// Module: crate::tls13
// Provides: {"impl_1794"}
// Dependencies: {}
impl Suite for Tls13CipherSuite { fn client_handler (& self) -> & 'static dyn crate :: client :: ClientHandler < Self > { self . protocol_version . client } fn server_handler (& self) -> & 'static dyn crate :: server :: ServerHandler < Self > { self . protocol_version . server } # [doc = " Does this suite support the `proto` protocol?"] # [doc = ""] # [doc = " All TLS1.3 suites support TCP-TLS. QUIC support is conditional on `quic` slot."] fn usable_for_protocol (& self , proto : Protocol) -> bool { match proto { Protocol :: Tcp => true , Protocol :: Quic => self . quic . is_some () , } } fn usable_for_signature_scheme (& self , scheme : SignatureScheme) -> bool { scheme . supported_in_tls13 () } fn common (& self) -> & CipherSuiteCommon { & self . common } const VERSION : ProtocolVersion = ProtocolVersion :: TLSv1_3 ; }
};
}
