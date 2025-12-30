// Generated macro for impl_2667 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2667 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2667"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsRawFd for OwnedFd { # [inline] fn as_raw_fd (& self) -> RawFd { self . fd . as_inner () } }
};
}
