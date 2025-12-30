// Generated macro for impl_2638 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2638 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2638"}
// Dependencies: {}
# [stable (feature = "asraw_stdio" , since = "1.21.0")] impl AsRawFd for io :: Stderr { # [inline] fn as_raw_fd (& self) -> RawFd { libc :: STDERR_FILENO } }
};
}
