// Generated macro for impl_879 (impl)
macro_rules! Depcrate_tlsimpl_879 {
() => {
// Module: crate::tls
// Provides: {"impl_879"}
// Dependencies: {}
impl TlsStream { fn new (stream : AddrStream , config : Arc < ServerConfig >) -> TlsStream { let remote_addr = stream . remote_addr () ; let accept = tokio_rustls :: TlsAcceptor :: from (config) . accept (stream) ; TlsStream { state : State :: Handshaking (accept) , remote_addr , } } }
};
}
