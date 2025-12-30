// Generated macro for impl_1560 (impl)
macro_rules! Depcrate_os_unix_net_listenerimpl_1560 {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"impl_1560"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl IntoRawFd for UnixListener { # [inline] fn into_raw_fd (self) -> RawFd { self . 0 . into_inner () . into_inner () . into_raw_fd () } }
};
}
