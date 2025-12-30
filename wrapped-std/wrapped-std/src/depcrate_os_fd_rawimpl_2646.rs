// Generated macro for impl_2646 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2646 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2646"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] # [cfg (not (target_os = "trusty"))] impl AsRawFd for io :: PipeReader { fn as_raw_fd (& self) -> RawFd { self . 0 . as_raw_fd () } }
};
}
