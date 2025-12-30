// Generated macro for impl_96 (impl)
macro_rules! Depcrate_writeimpl_96 {
() => {
// Module: crate::write
// Provides: {"impl_96"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < W : AsyncWrite > AsyncWrite for XzDecoder < W > { fn shutdown (& mut self) -> Poll < () , io :: Error > { try_nb ! (self . try_finish ()) ; self . get_mut () . shutdown () } }
};
}
