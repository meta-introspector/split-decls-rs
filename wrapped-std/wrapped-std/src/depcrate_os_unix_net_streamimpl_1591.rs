// Generated macro for impl_1591 (impl)
macro_rules! Depcrate_os_unix_net_streamimpl_1591 {
() => {
// Module: crate::os::unix::net::stream
// Provides: {"impl_1591"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl FromRawFd for UnixStream { # [inline] unsafe fn from_raw_fd (fd : RawFd) -> UnixStream { UnixStream (Socket :: from_inner (FromInner :: from_inner (OwnedFd :: from_raw_fd (fd)))) } }
};
}
