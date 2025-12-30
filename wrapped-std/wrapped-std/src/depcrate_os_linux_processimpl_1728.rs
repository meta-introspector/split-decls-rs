// Generated macro for impl_1728 (impl)
macro_rules! Depcrate_os_linux_processimpl_1728 {
() => {
// Module: crate::os::linux::process
// Provides: {"impl_1728"}
// Dependencies: {}
impl FromRawFd for PidFd { unsafe fn from_raw_fd (fd : RawFd) -> Self { Self :: from_inner (InnerPidFd :: from_raw_fd (fd)) } }
};
}
