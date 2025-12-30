// Generated macro for impl_858 (impl)
macro_rules! Depcrate_net_addrimpl_858 {
() => {
// Module: crate::net::addr
// Provides: {"impl_858"}
// Dependencies: {}
unsafe impl SocketAddrArg for SocketAddrV6 { unsafe fn with_sockaddr < R > (& self , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { call_with_sockaddr (& encode_sockaddr_v6 (self) , f) } }
};
}
