// Generated macro for impl_91 (impl)
macro_rules! Depcrate_http3_driver_connectionimpl_91 {
() => {
// Module: crate::http3::driver::connection
// Provides: {"impl_91"}
// Dependencies: {}
impl < H : DriverHooks > ShutdownConnection for H3Connection < H > { # [inline] fn poll_shutdown (& mut self , _cx : & mut std :: task :: Context ,) -> Poll < std :: io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
