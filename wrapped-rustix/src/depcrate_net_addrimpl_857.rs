// Generated macro for impl_857 (impl)
macro_rules! Depcrate_net_addrimpl_857 {
() => {
// Module: crate::net::addr
// Provides: {"impl_857"}
// Dependencies: {}
unsafe impl SocketAddrArg for SocketAddrV4 { unsafe fn with_sockaddr < R > (& self , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { call_with_sockaddr (& encode_sockaddr_v4 (self) , f) } }
};
}
