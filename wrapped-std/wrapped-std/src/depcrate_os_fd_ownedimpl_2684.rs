// Generated macro for impl_2684 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2684 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2684"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] # [cfg (not (target_os = "trusty"))] impl From < crate :: net :: TcpStream > for OwnedFd { # [doc = " Takes ownership of a [`TcpStream`](crate::net::TcpStream)'s socket file descriptor."] # [inline] fn from (tcp_stream : crate :: net :: TcpStream) -> OwnedFd { tcp_stream . into_inner () . into_socket () . into_inner () . into_inner () . into () } }
};
}
