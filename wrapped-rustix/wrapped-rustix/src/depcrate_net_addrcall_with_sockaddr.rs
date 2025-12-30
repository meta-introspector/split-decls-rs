// Generated macro for call_with_sockaddr (function)
macro_rules! Depcrate_net_addrcall_with_sockaddr {
() => {
// Module: crate::net::addr
// Provides: {"call_with_sockaddr"}
// Dependencies: {}
# [doc = " Helper for implementing `SocketAddrArg::with_sockaddr`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This calls `f` with a pointer to an object it has a reference to, with the"] # [doc = " and the length of that object, so they'll be valid for the duration of the"] # [doc = " call."] pub (crate) unsafe fn call_with_sockaddr < A , R > (addr : & A , f : impl FnOnce (* const SocketAddrOpaque , SocketAddrLen) -> R ,) -> R { let ptr = as_ptr (addr) . cast () ; let len = size_of :: < A > () as SocketAddrLen ; f (ptr , len) }
};
}
