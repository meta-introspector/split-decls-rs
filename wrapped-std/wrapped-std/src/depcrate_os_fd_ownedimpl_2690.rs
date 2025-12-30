// Generated macro for impl_2690 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2690 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2690"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] # [cfg (not (target_os = "trusty"))] impl From < crate :: net :: UdpSocket > for OwnedFd { # [doc = " Takes ownership of a [`UdpSocket`](crate::net::UdpSocket)'s file descriptor."] # [inline] fn from (udp_socket : crate :: net :: UdpSocket) -> OwnedFd { udp_socket . into_inner () . into_socket () . into_inner () . into_inner () . into () } }
};
}
