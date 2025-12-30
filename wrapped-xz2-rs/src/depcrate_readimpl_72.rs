// Generated macro for impl_72 (impl)
macro_rules! Depcrate_readimpl_72 {
() => {
// Module: crate::read
// Provides: {"impl_72"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < R : AsyncWrite + Read > AsyncWrite for XzEncoder < R > { fn shutdown (& mut self) -> Poll < () , io :: Error > { self . get_mut () . shutdown () } }
};
}
