// Generated macro for impl_2639 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2639 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2639"}
// Dependencies: {}
# [stable (feature = "asraw_stdio_locks" , since = "1.35.0")] # [cfg (not (target_os = "trusty"))] impl < 'a > AsRawFd for io :: StdinLock < 'a > { # [inline] fn as_raw_fd (& self) -> RawFd { libc :: STDIN_FILENO } }
};
}
