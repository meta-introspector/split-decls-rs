// Generated macro for impl_1558 (impl)
macro_rules! Depcrate_os_unix_net_listenerimpl_1558 {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"impl_1558"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl AsRawFd for UnixListener { # [inline] fn as_raw_fd (& self) -> RawFd { self . 0 . as_inner () . as_raw_fd () } }
};
}
