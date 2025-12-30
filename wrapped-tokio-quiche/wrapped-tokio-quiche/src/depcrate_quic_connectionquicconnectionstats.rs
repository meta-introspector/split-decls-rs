// Generated macro for QuicConnectionStats (struct)
macro_rules! Depcrate_quic_connectionQuicConnectionStats {
() => {
// Module: crate::quic::connection
// Provides: {"QuicConnectionStats"}
// Dependencies: {}
# [doc = " Wrapper for connection statistics recorded by [quiche]."] # [derive (Debug)] pub struct QuicConnectionStats { # [doc = " Aggregate connection statistics across all paths."] pub stats : quiche :: Stats , # [doc = " Specific statistics about the connection's active path."] pub path_stats : Option < quiche :: PathStats > , }
};
}
