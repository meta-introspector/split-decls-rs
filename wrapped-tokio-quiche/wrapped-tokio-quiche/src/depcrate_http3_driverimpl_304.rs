// Generated macro for impl_304 (impl)
macro_rules! Depcrate_http3_driverimpl_304 {
() => {
// Module: crate::http3::driver
// Provides: {"impl_304"}
// Dependencies: {}
impl < H : DriverHooks > Drop for H3Driver < H > { fn drop (& mut self) { for stream in self . stream_map . values () { stream . audit_stats . set_recvd_stream_fin (StreamClosureKind :: Implicit) ; } } }
};
}
