// Generated macro for impl_92 (impl)
macro_rules! Depcrate_http3_driver_connectionimpl_92 {
() => {
// Module: crate::http3::driver::connection
// Provides: {"impl_92"}
// Dependencies: {}
impl < H : DriverHooks > AsSocketStats for H3Connection < H > { # [inline] fn as_socket_stats (& self) -> SocketStats { self . quic_connection . as_socket_stats () } # [inline] fn as_quic_stats (& self) -> Option < & Arc < QuicAuditStats > > { self . quic_connection . as_quic_stats () } }
};
}
