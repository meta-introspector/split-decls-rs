// Generated macro for QuicConnection (struct)
macro_rules! Depcrate_quic_connectionQuicConnection {
() => {
// Module: crate::quic::connection
// Provides: {"QuicConnection"}
// Dependencies: {}
# [doc = " Metadata about an established QUIC connection."] # [doc = ""] # [doc = " While this struct allows access to some facets of a QUIC connection, it"] # [doc = " notably does not represent the [quiche::Connection] itself. The crate"] # [doc = " handles most interactions with [quiche] internally in a worker task. Users"] # [doc = " can only access the connection directly via their [`ApplicationOverQuic`]"] # [doc = " implementation."] # [doc = ""] # [doc = " See the [module-level docs](crate::quic) for an overview of how a QUIC"] # [doc = " connection is handled internally."] pub struct QuicConnection { local_addr : SocketAddr , peer_addr : SocketAddr , audit_log_stats : Arc < QuicAuditStats > , stats : QuicConnectionStatsShared , scid : ConnectionId < 'static > , }
};
}
