// Generated macro for impl_2650 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2650 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2650"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] # [cfg (not (target_os = "trusty"))] impl FromRawFd for io :: PipeWriter { unsafe fn from_raw_fd (raw_fd : RawFd) -> Self { Self :: from_inner (unsafe { FromRawFd :: from_raw_fd (raw_fd) }) } }
};
}
