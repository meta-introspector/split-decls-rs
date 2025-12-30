// Generated macro for impl_859 (impl)
macro_rules! Depcrate_net_addrimpl_859 {
() => {
// Module: crate::net::addr
// Provides: {"impl_859"}
// Dependencies: {}
# [cfg (unix)] unsafe impl SocketAddrArg for SocketAddrUnix { unsafe fn with_sockaddr < R > (& self , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { f (as_ptr (& self . unix) . cast () , self . addr_len ()) } }
};
}
