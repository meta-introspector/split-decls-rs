// Generated macro for impl_57 (impl)
macro_rules! Depcrate_bufreadimpl_57 {
() => {
// Module: crate::bufread
// Provides: {"impl_57"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < R : AsyncWrite > AsyncWrite for XzDecoder < R > { fn shutdown (& mut self) -> Poll < () , io :: Error > { self . get_mut () . shutdown () } }
};
}
