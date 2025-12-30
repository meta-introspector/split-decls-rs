// Generated macro for WriteState (struct)
macro_rules! Depcrate_quic_io_workerWriteState {
() => {
// Module: crate::quic::io::worker
// Provides: {"WriteState"}
// Dependencies: {}
# [derive (Default)] pub (crate) struct WriteState { conn_established : bool , bytes_written : usize , segment_size : usize , num_pkts : usize , tx_time : Option < Instant > , has_pending_data : bool , next_release_time : Option < Instant > , selected_path : Option < (SocketAddr , SocketAddr) > , pending_paths : quiche :: SocketAddrIter , }
};
}
