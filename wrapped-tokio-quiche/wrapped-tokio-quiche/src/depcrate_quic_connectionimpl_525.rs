// Generated macro for impl_525 (impl)
macro_rules! Depcrate_quic_connectionimpl_525 {
() => {
// Module: crate::quic::connection
// Provides: {"impl_525"}
// Dependencies: {}
impl ShutdownConnection for QuicConnection { # [inline] fn poll_shutdown (& mut self , _cx : & mut std :: task :: Context ,) -> std :: task :: Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
