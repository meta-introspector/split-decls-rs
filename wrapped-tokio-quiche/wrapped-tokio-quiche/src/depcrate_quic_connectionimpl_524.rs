// Generated macro for impl_524 (impl)
macro_rules! Depcrate_quic_connectionimpl_524 {
() => {
// Module: crate::quic::connection
// Provides: {"impl_524"}
// Dependencies: {}
impl < Tx , M > ShutdownConnection for InitialQuicConnection < Tx , M > where Tx : DatagramSocketSend + Send + 'static + ? Sized , M : Metrics , { # [inline] fn poll_shutdown (& mut self , _cx : & mut std :: task :: Context ,) -> std :: task :: Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
