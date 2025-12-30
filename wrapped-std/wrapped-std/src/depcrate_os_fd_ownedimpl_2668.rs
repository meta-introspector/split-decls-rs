// Generated macro for impl_2668 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2668 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2668"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl IntoRawFd for OwnedFd { # [inline] fn into_raw_fd (self) -> RawFd { ManuallyDrop :: new (self) . fd . as_inner () } }
};
}
