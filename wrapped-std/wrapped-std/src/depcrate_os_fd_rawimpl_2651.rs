// Generated macro for impl_2651 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2651 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2651"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] # [cfg (not (target_os = "trusty"))] impl IntoRawFd for io :: PipeWriter { fn into_raw_fd (self) -> RawFd { self . 0 . into_raw_fd () } }
};
}
