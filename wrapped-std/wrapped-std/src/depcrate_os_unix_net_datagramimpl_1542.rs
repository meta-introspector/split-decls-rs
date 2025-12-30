// Generated macro for impl_1542 (impl)
macro_rules! Depcrate_os_unix_net_datagramimpl_1542 {
() => {
// Module: crate::os::unix::net::datagram
// Provides: {"impl_1542"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl IntoRawFd for UnixDatagram { # [inline] fn into_raw_fd (self) -> RawFd { self . 0 . into_inner () . into_inner () . into_raw_fd () } }
};
}
