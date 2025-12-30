// Generated macro for impl_1674 (impl)
macro_rules! Depcrate_os_unix_processimpl_1674 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1674"}
// Dependencies: {}
# [stable (feature = "process_extensions" , since = "1.2.0")] impl AsRawFd for process :: ChildStderr { # [inline] fn as_raw_fd (& self) -> RawFd { self . as_inner () . as_raw_fd () } }
};
}
