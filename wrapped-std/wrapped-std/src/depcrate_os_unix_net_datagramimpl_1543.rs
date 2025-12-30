// Generated macro for impl_1543 (impl)
macro_rules! Depcrate_os_unix_net_datagramimpl_1543 {
() => {
// Module: crate::os::unix::net::datagram
// Provides: {"impl_1543"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsFd for UnixDatagram { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 . as_inner () . as_fd () } }
};
}
