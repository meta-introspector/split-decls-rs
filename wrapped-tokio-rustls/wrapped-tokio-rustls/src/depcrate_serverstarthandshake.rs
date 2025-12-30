// Generated macro for StartHandshake (struct)
macro_rules! Depcrate_serverStartHandshake {
() => {
// Module: crate::server
// Provides: {"StartHandshake"}
// Dependencies: {}
# [doc = " An incoming connection received through [`LazyConfigAcceptor`]."] # [doc = ""] # [doc = " This contains the generic `IO` asynchronous transport,"] # [doc = " [`ClientHello`](rustls::server::ClientHello) data,"] # [doc = " and all the state required to continue the TLS handshake (e.g. via"] # [doc = " [`StartHandshake::into_stream`])."] # [non_exhaustive] # [derive (Debug)] pub struct StartHandshake < IO > { pub accepted : rustls :: server :: Accepted , pub io : IO , }
};
}
