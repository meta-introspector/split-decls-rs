// Generated macro for impl_1545 (impl)
macro_rules! Depcrate_os_unix_net_datagramimpl_1545 {
() => {
// Module: crate::os::unix::net::datagram
// Provides: {"impl_1545"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < OwnedFd > for UnixDatagram { # [inline] fn from (owned : OwnedFd) -> Self { unsafe { Self :: from_raw_fd (owned . into_raw_fd ()) } } }
};
}
