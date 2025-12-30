// Generated macro for TlsState (enum)
macro_rules! Depcrate_commonTlsState {
() => {
// Module: crate::common
// Provides: {"TlsState"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum TlsState { # [cfg (feature = "early-data")] EarlyData (usize , Vec < u8 >) , Stream , ReadShutdown , WriteShutdown , FullyShutdown , }
};
}
