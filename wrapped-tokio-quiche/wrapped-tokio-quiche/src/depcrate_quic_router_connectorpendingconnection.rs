// Generated macro for PendingConnection (struct)
macro_rules! Depcrate_quic_router_connectorPendingConnection {
() => {
// Module: crate::quic::router::connector
// Provides: {"PendingConnection"}
// Dependencies: {}
# [doc = " A [`PendingConnection`] holds an internal [`quiche::Connection`] and an"] # [doc = " optional timeout [`Key`]."] struct PendingConnection { conn : QuicheConnection , timeout_key : Option < Key > , handshake_start_time : Instant , }
};
}
