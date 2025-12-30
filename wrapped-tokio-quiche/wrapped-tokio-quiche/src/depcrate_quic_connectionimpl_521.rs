// Generated macro for impl_521 (impl)
macro_rules! Depcrate_quic_connectionimpl_521 {
() => {
// Module: crate::quic::connection
// Provides: {"impl_521"}
// Dependencies: {}
impl QuicConnection { # [doc = " The local address this connection listens on."] # [inline] pub fn local_addr (& self) -> SocketAddr { self . local_addr } # [doc = " The remote address for this connection."] # [inline] pub fn peer_addr (& self) -> SocketAddr { self . peer_addr } # [doc = " A handle to the [`QuicAuditStats`] for this connection."] # [doc = ""] # [doc = " # Note"] # [doc = " These stats are updated during the lifetime of the connection."] # [doc = " The getter exists to grab a handle early on, which can then"] # [doc = " be stowed away and read out after the connection has closed."] # [inline] pub fn audit_log_stats (& self) -> & Arc < QuicAuditStats > { & self . audit_log_stats } # [doc = " A handle to the [`QuicConnectionStats`] for this connection."] # [doc = ""] # [doc = " # Note"] # [doc = " Initially, these stats represent the state when the [quiche::Connection]"] # [doc = " was created. They are updated when the connection is closed, so this"] # [doc = " getter exists primarily to grab a handle early on."] # [inline] pub fn stats (& self) -> & QuicConnectionStatsShared { & self . stats } # [doc = " The QUIC source connection ID used by this connection."] # [inline] pub fn scid (& self) -> & ConnectionId < 'static > { & self . scid } }
};
}
