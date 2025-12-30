// Generated macro for impl_1717 (impl)
macro_rules! Depcrate_tls12impl_1717 {
() => {
// Module: crate::tls12
// Provides: {"impl_1717"}
// Dependencies: {}
impl Suite for Tls12CipherSuite { fn client_handler (& self) -> & 'static dyn crate :: client :: ClientHandler < Self > { self . protocol_version . client } fn server_handler (& self) -> & 'static dyn crate :: server :: ServerHandler < Self > { self . protocol_version . server } # [doc = " Does this suite support the `proto` protocol?"] # [doc = ""] # [doc = " All TLS1.2 suites support TCP-TLS. No TLS1.2 suites support QUIC."] fn usable_for_protocol (& self , proto : Protocol) -> bool { matches ! (proto , Protocol :: Tcp) } # [doc = " Say if the given `KeyExchangeAlgorithm` is supported by this cipher suite."] fn usable_for_kx_algorithm (& self , kxa : KeyExchangeAlgorithm) -> bool { self . kx == kxa } # [doc = " Return true if this suite is usable for a key only offering `sig_alg`"] # [doc = " signatures."] fn usable_for_signature_scheme (& self , scheme : SignatureScheme) -> bool { self . sign . iter () . any (| s | s . algorithm () == scheme . algorithm ()) } fn common (& self) -> & CipherSuiteCommon { & self . common } const VERSION : ProtocolVersion = ProtocolVersion :: TLSv1_2 ; }
};
}
