// Generated macro for impl_856 (impl)
macro_rules! Depcrate_net_addrimpl_856 {
() => {
// Module: crate::net::addr
// Provides: {"impl_856"}
// Dependencies: {}
unsafe impl SocketAddrArg for SocketAddr { unsafe fn with_sockaddr < R > (& self , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { match self { Self :: V4 (v4) => v4 . with_sockaddr (f) , Self :: V6 (v6) => v6 . with_sockaddr (f) , } } }
};
}
