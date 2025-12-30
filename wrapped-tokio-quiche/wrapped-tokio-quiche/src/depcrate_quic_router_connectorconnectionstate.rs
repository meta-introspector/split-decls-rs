// Generated macro for ConnectionState (enum)
macro_rules! Depcrate_quic_router_connectorConnectionState {
() => {
// Module: crate::quic::router::connector
// Provides: {"ConnectionState"}
// Dependencies: {}
# [doc = " State the connecting connection is in."] enum ConnectionState { # [doc = " Connection hasn't had any initials sent for it"] Queued (QuicheConnection) , # [doc = " It's currently in a QUIC handshake"] Pending (PendingConnection) , # [doc = " It's been returned to the"] # [doc = " [`InboundPacketRouter`](super::InboundPacketRouter)."] Returned , }
};
}
