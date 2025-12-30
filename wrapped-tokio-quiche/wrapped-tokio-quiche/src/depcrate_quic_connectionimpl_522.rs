// Generated macro for impl_522 (impl)
macro_rules! Depcrate_quic_connectionimpl_522 {
() => {
// Module: crate::quic::connection
// Provides: {"impl_522"}
// Dependencies: {}
impl AsSocketStats for QuicConnection { # [inline] fn as_socket_stats (& self) -> SocketStats { self . stats . lock () . unwrap () . as_socket_stats () } # [inline] fn as_quic_stats (& self) -> Option < & Arc < QuicAuditStats > > { Some (& self . audit_log_stats) } }
};
}
