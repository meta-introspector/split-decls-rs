// Generated macro for impl_1559 (impl)
macro_rules! Depcrate_os_unix_net_listenerimpl_1559 {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"impl_1559"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl FromRawFd for UnixListener { # [inline] unsafe fn from_raw_fd (fd : RawFd) -> UnixListener { UnixListener (Socket :: from_inner (FromInner :: from_inner (OwnedFd :: from_raw_fd (fd)))) } }
};
}
