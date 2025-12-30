// Generated macro for impl_2640 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2640 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2640"}
// Dependencies: {}
# [stable (feature = "asraw_stdio_locks" , since = "1.35.0")] impl < 'a > AsRawFd for io :: StdoutLock < 'a > { # [inline] fn as_raw_fd (& self) -> RawFd { libc :: STDOUT_FILENO } }
};
}
