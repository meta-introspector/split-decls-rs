// Generated macro for impl_2641 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2641 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2641"}
// Dependencies: {}
# [stable (feature = "asraw_stdio_locks" , since = "1.35.0")] impl < 'a > AsRawFd for io :: StderrLock < 'a > { # [inline] fn as_raw_fd (& self) -> RawFd { libc :: STDERR_FILENO } }
};
}
