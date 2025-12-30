// Generated macro for QuicConnectionParams (struct)
macro_rules! Depcrate_quic_connectionQuicConnectionParams {
() => {
// Module: crate::quic::connection
// Provides: {"QuicConnectionParams"}
// Dependencies: {}
pub (crate) struct QuicConnectionParams < Tx , M > where Tx : DatagramSocketSend + Send + 'static + ? Sized , M : Metrics , { pub writer_cfg : WriterConfig , pub initial_pkt : Option < Incoming > , pub shutdown_tx : mpsc :: Sender < () > , pub conn_map_cmd_tx : mpsc :: UnboundedSender < ConnectionMapCommand > , pub scid : ConnectionId < 'static > , pub metrics : M , # [cfg (feature = "perf-quic-listener-metrics")] pub init_rx_time : Option < SystemTime > , pub handshake_info : HandshakeInfo , pub quiche_conn : QuicheConnection , pub socket : Arc < Tx > , pub local_addr : SocketAddr , pub peer_addr : SocketAddr , }
};
}
