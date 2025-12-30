// Generated macro for impl_1541 (impl)
macro_rules! Depcrate_os_unix_net_datagramimpl_1541 {
() => {
// Module: crate::os::unix::net::datagram
// Provides: {"impl_1541"}
// Dependencies: {}
# [stable (feature = "unix_socket" , since = "1.10.0")] impl FromRawFd for UnixDatagram { # [inline] unsafe fn from_raw_fd (fd : RawFd) -> UnixDatagram { UnixDatagram (Socket :: from_inner (FromInner :: from_inner (OwnedFd :: from_raw_fd (fd)))) } }
};
}
