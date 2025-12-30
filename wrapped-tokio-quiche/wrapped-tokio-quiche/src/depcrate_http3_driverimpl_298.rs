// Generated macro for impl_298 (impl)
macro_rules! Depcrate_http3_driverimpl_298 {
() => {
// Module: crate::http3::driver
// Provides: {"impl_298"}
// Dependencies: {}
impl OutboundFrame { # [doc = " Creates a body frame with the provided buffer."] pub fn body (body : PooledBuf , fin : bool) -> Self { # [cfg (feature = "zero-copy")] let body = crate :: buf_factory :: QuicheBuf :: new (body) ; OutboundFrame :: Body (body , fin) } }
};
}
