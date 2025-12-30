// Generated macro for impl_1595 (impl)
macro_rules! Depcrate_os_unix_net_streamimpl_1595 {
() => {
// Module: crate::os::unix::net::stream
// Provides: {"impl_1595"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < OwnedFd > for UnixStream { # [inline] fn from (owned : OwnedFd) -> Self { unsafe { Self :: from_raw_fd (owned . into_raw_fd ()) } } }
};
}
