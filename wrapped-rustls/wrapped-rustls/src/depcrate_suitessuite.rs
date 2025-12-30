// Generated macro for Suite (trait)
macro_rules! Depcrate_suitesSuite {
() => {
// Module: crate::suites
// Provides: {"Suite"}
// Dependencies: {}
pub (crate) trait Suite : fmt :: Debug { fn client_handler (& self) -> & 'static dyn crate :: client :: ClientHandler < Self > ; fn server_handler (& self) -> & 'static dyn crate :: server :: ServerHandler < Self > ; fn usable_for_protocol (& self , proto : Protocol) -> bool ; fn usable_for_signature_scheme (& self , _scheme : SignatureScheme) -> bool ; fn usable_for_kx_algorithm (& self , _kxa : KeyExchangeAlgorithm) -> bool { true } fn suite (& self) -> CipherSuite { self . common () . suite } fn common (& self) -> & CipherSuiteCommon ; const VERSION : ProtocolVersion ; }
};
}
