// Generated macro for impl_514 (impl)
macro_rules! Depcrate_quic_connectionimpl_514 {
() => {
// Module: crate::quic::connection
// Provides: {"impl_514"}
// Dependencies: {}
impl QuicConnectionStats { pub (crate) fn from_conn (qconn : & QuicheConnection) -> Self { Self { stats : qconn . stats () , path_stats : qconn . path_stats () . next () , } } fn startup_exit_to_socket_stats (value : quiche :: StartupExit ,) -> datagram_socket :: StartupExit { let reason = match value . reason { quiche :: StartupExitReason :: Loss => datagram_socket :: StartupExitReason :: Loss , quiche :: StartupExitReason :: BandwidthPlateau => datagram_socket :: StartupExitReason :: BandwidthPlateau , quiche :: StartupExitReason :: PersistentQueue => datagram_socket :: StartupExitReason :: PersistentQueue , } ; datagram_socket :: StartupExit { cwnd : value . cwnd , bandwidth : value . bandwidth , reason , } } }
};
}
