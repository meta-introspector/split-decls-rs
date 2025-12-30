// Generated macro for impl_1544 (impl)
macro_rules! Depcrate_os_unix_net_datagramimpl_1544 {
() => {
// Module: crate::os::unix::net::datagram
// Provides: {"impl_1544"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < UnixDatagram > for OwnedFd { # [doc = " Takes ownership of a [`UnixDatagram`]'s socket file descriptor."] # [inline] fn from (unix_datagram : UnixDatagram) -> OwnedFd { unsafe { OwnedFd :: from_raw_fd (unix_datagram . into_raw_fd ()) } } }
};
}
