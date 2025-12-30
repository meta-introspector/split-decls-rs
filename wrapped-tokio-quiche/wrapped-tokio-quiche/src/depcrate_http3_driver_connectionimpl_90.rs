// Generated macro for impl_90 (impl)
macro_rules! Depcrate_http3_driver_connectionimpl_90 {
() => {
// Module: crate::http3::driver::connection
// Provides: {"impl_90"}
// Dependencies: {}
impl < H : DriverHooks > H3Connection < H > { # [doc = " Bundles `quic_connection` and `h3_controller` into a new [H3Connection]."] pub fn new (quic_connection : QuicConnection , h3_controller : H3Controller < H > ,) -> Self { Self { quic_connection , h3_controller , } } # [doc = " The local address this connection listens on."] pub fn local_addr (& self) -> SocketAddr { self . quic_connection . local_addr () } # [doc = " The remote address for this connection."] pub fn peer_addr (& self) -> SocketAddr { self . quic_connection . peer_addr () } # [doc = " The [QuicConnection]'s audit stats."] pub fn audit_log_stats (& self) -> & Arc < QuicAuditStats > { self . quic_connection . audit_log_stats () } # [doc = " The [QuicConnection]'s [`quiche`] stats."] pub fn stats (& self) -> & Arc < Mutex < QuicConnectionStats > > { self . quic_connection . stats () } # [doc = " The [QuicConnection]'s source connection ID."] pub fn scid (& self) -> & ConnectionId < 'static > { self . quic_connection . scid () } }
};
}
