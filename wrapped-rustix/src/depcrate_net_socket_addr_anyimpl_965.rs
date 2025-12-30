// Generated macro for impl_965 (impl)
macro_rules! Depcrate_net_socket_addr_anyimpl_965 {
() => {
// Module: crate::net::socket_addr_any
// Provides: {"impl_965"}
// Dependencies: {}
unsafe impl SocketAddrArg for SocketAddrAny { unsafe fn with_sockaddr < R > (& self , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { f (self . as_ptr () . cast () , self . addr_len ()) } }
};
}
