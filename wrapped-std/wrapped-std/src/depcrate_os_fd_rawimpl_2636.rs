// Generated macro for impl_2636 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2636 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2636"}
// Dependencies: {}
# [stable (feature = "asraw_stdio" , since = "1.21.0")] # [cfg (not (target_os = "trusty"))] impl AsRawFd for io :: Stdin { # [inline] fn as_raw_fd (& self) -> RawFd { libc :: STDIN_FILENO } }
};
}
