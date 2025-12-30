// Generated macro for impl_77 (impl)
macro_rules! Depcrate_readimpl_77 {
() => {
// Module: crate::read
// Provides: {"impl_77"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < R : AsyncWrite + Read > AsyncWrite for XzDecoder < R > { fn shutdown (& mut self) -> Poll < () , io :: Error > { self . get_mut () . shutdown () } }
};
}
