// Generated macro for impl_2637 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2637 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2637"}
// Dependencies: {}
# [stable (feature = "asraw_stdio" , since = "1.21.0")] impl AsRawFd for io :: Stdout { # [inline] fn as_raw_fd (& self) -> RawFd { libc :: STDOUT_FILENO } }
};
}
