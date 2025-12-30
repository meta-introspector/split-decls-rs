// Generated macro for impl_51 (impl)
macro_rules! Depcrate_bufreadimpl_51 {
() => {
// Module: crate::bufread
// Provides: {"impl_51"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < R : AsyncWrite > AsyncWrite for XzEncoder < R > { fn shutdown (& mut self) -> Poll < () , io :: Error > { self . get_mut () . shutdown () } }
};
}
