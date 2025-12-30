// Generated macro for ClientConnector (struct)
macro_rules! Depcrate_quic_router_connectorClientConnector {
() => {
// Module: crate::quic::router::connector
// Provides: {"ClientConnector"}
// Dependencies: {}
# [doc = " A [`ClientConnector`] manages client-initiated [`quiche::Connection`]s. When"] # [doc = " a connection is established, this struct returns the connection to the"] # [doc = " [`InboundPacketRouter`](super::InboundPacketRouter) for further processing."] pub (crate) struct ClientConnector < Tx > { socket_tx : MaybeConnectedSocket < Arc < Tx > > , connection : ConnectionState , timeout_queue : DelayQueue < ConnectionId < 'static > > , }
};
}
