// Generated macro for impl_1675 (impl)
macro_rules! Depcrate_os_unix_processimpl_1675 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1675"}
// Dependencies: {}
# [stable (feature = "into_raw_os" , since = "1.4.0")] impl IntoRawFd for process :: ChildStdin { # [inline] fn into_raw_fd (self) -> RawFd { self . into_inner () . into_inner () . into_raw_fd () } }
};
}
