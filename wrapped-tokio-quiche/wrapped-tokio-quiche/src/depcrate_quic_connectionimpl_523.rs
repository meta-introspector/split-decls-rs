// Generated macro for impl_523 (impl)
macro_rules! Depcrate_quic_connectionimpl_523 {
() => {
// Module: crate::quic::connection
// Provides: {"impl_523"}
// Dependencies: {}
impl < Tx , M > AsSocketStats for InitialQuicConnection < Tx , M > where Tx : DatagramSocketSend + Send + 'static + ? Sized , M : Metrics , { # [inline] fn as_socket_stats (& self) -> SocketStats { self . stats . lock () . unwrap () . as_socket_stats () } # [inline] fn as_quic_stats (& self) -> Option < & Arc < QuicAuditStats > > { Some (& self . audit_log_stats) } }
};
}
