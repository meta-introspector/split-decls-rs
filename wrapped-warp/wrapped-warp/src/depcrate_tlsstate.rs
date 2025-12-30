// Generated macro for State (enum)
macro_rules! Depcrate_tlsState {
() => {
// Module: crate::tls
// Provides: {"State"}
// Dependencies: {}
enum State { Handshaking (tokio_rustls :: Accept < AddrStream >) , Streaming (tokio_rustls :: server :: TlsStream < AddrStream >) , }
};
}
