// Generated macro for impl_90 (impl)
macro_rules! Depcrate_writeimpl_90 {
() => {
// Module: crate::write
// Provides: {"impl_90"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < W : AsyncWrite > AsyncWrite for XzEncoder < W > { fn shutdown (& mut self) -> Poll < () , io :: Error > { try_nb ! (self . try_finish ()) ; self . get_mut () . shutdown () } }
};
}
