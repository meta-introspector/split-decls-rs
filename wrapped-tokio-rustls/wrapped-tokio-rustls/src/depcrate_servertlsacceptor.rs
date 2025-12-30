// Generated macro for TlsAcceptor (struct)
macro_rules! Depcrate_serverTlsAcceptor {
() => {
// Module: crate::server
// Provides: {"TlsAcceptor"}
// Dependencies: {}
# [doc = " A wrapper around a `rustls::ServerConfig`, providing an async `accept` method."] # [derive (Clone)] pub struct TlsAcceptor { inner : Arc < ServerConfig > , }
};
}
