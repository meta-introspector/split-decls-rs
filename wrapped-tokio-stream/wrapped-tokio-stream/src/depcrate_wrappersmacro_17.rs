// Generated macro for macro_17 (macro)
macro_rules! Depcrate_wrappersmacro_17 {
() => {
// Module: crate::wrappers
// Provides: {"macro_17"}
// Dependencies: {}
cfg_net ! { # [cfg (not (loom))] mod tcp_listener ; # [cfg (not (loom))] pub use tcp_listener :: TcpListenerStream ; # [cfg (all (unix , not (loom)))] mod unix_listener ; # [cfg (all (unix , not (loom)))] pub use unix_listener :: UnixListenerStream ; }
};
}
