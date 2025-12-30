// Generated macro for impl_1677 (impl)
macro_rules! Depcrate_os_unix_processimpl_1677 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1677"}
// Dependencies: {}
# [stable (feature = "into_raw_os" , since = "1.4.0")] impl IntoRawFd for process :: ChildStderr { # [inline] fn into_raw_fd (self) -> RawFd { self . into_inner () . into_inner () . into_raw_fd () } }
};
}
