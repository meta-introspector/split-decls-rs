// Generated macro for HandshakeError (enum)
macro_rules! Depcrate_quic_connection_errorHandshakeError {
() => {
// Module: crate::quic::connection::error
// Provides: {"HandshakeError"}
// Dependencies: {}
# [doc = " Additional error types that can occur during a QUIC handshake."] # [doc = ""] # [doc = " Protocol errors are returned directly as [`quiche::Error`] values."] # [non_exhaustive] # [derive (Debug , Clone , thiserror :: Error)] pub enum HandshakeError { # [doc = " The configured handshake timeout has expired."] # [error ("handshake timeout expired")] Timeout , # [doc = " The connection was closed while handshaking, for example by the peer."] # [error ("connection closed during Handshake stage")] ConnectionClosed , }
};
}
