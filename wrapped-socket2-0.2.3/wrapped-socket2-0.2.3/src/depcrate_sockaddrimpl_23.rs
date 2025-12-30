// Generated macro for impl_23 (impl)
macro_rules! Depcrate_sockaddrimpl_23 {
() => {
// Module: crate::sockaddr
// Provides: {"impl_23"}
// Dependencies: {}
impl From < SocketAddr > for SockAddr { fn from (addr : SocketAddr) -> SockAddr { match addr { SocketAddr :: V4 (addr) => addr . into () , SocketAddr :: V6 (addr) => addr . into () , } } }
};
}
