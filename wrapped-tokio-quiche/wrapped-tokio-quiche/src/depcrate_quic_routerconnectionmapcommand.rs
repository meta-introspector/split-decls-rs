// Generated macro for ConnectionMapCommand (enum)
macro_rules! Depcrate_quic_routerConnectionMapCommand {
() => {
// Module: crate::quic::router
// Provides: {"ConnectionMapCommand"}
// Dependencies: {}
# [doc = " A message to the listener notifiying a mapping for a connection should be"] # [doc = " removed."] pub enum ConnectionMapCommand { UnmapCid (ConnectionId < 'static >) , RemoveScid (ConnectionId < 'static >) , }
};
}
