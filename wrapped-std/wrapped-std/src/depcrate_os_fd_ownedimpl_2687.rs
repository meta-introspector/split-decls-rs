// Generated macro for impl_2687 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2687 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2687"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] # [cfg (not (target_os = "trusty"))] impl From < crate :: net :: TcpListener > for OwnedFd { # [doc = " Takes ownership of a [`TcpListener`](crate::net::TcpListener)'s socket file descriptor."] # [inline] fn from (tcp_listener : crate :: net :: TcpListener) -> OwnedFd { tcp_listener . into_inner () . into_socket () . into_inner () . into_inner () . into () } }
};
}
