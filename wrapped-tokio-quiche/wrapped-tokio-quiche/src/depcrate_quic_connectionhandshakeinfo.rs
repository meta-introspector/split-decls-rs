// Generated macro for HandshakeInfo (struct)
macro_rules! Depcrate_quic_connectionHandshakeInfo {
() => {
// Module: crate::quic::connection
// Provides: {"HandshakeInfo"}
// Dependencies: {}
# [doc = " Details about a connection's QUIC handshake."] # [derive (Debug , Clone)] pub struct HandshakeInfo { # [doc = " The time at which the connection was created."] start_time : Instant , # [doc = " The timeout before which the handshake must complete."] timeout : Option < Duration > , # [doc = " The real duration that the handshake took to complete."] time_handshake : Option < Duration > , }
};
}
