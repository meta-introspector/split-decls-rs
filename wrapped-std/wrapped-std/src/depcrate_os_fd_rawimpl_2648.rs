// Generated macro for impl_2648 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2648 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2648"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] # [cfg (not (target_os = "trusty"))] impl IntoRawFd for io :: PipeReader { fn into_raw_fd (self) -> RawFd { self . 0 . into_raw_fd () } }
};
}
