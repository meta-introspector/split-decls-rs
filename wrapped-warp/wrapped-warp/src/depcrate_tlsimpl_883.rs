// Generated macro for impl_883 (impl)
macro_rules! Depcrate_tlsimpl_883 {
() => {
// Module: crate::tls
// Provides: {"impl_883"}
// Dependencies: {}
impl TlsAcceptor { pub (crate) fn new (config : ServerConfig , incoming : AddrIncoming) -> TlsAcceptor { TlsAcceptor { config : Arc :: new (config) , incoming , } } }
};
}
