// Generated macro for WriterConfig (struct)
macro_rules! Depcrate_quic_io_workerWriterConfig {
() => {
// Module: crate::quic::io::worker
// Provides: {"WriterConfig"}
// Dependencies: {}
pub struct WriterConfig { pub pending_cid : Option < ConnectionId < 'static > > , pub peer_addr : SocketAddr , pub local_addr : SocketAddr , pub with_gso : bool , pub pacing_offload : bool , pub with_pktinfo : bool , }
};
}
