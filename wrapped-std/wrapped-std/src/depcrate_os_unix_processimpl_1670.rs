// Generated macro for impl_1670 (impl)
macro_rules! Depcrate_os_unix_processimpl_1670 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1670"}
// Dependencies: {}
# [stable (feature = "process_extensions" , since = "1.2.0")] impl FromRawFd for process :: Stdio { # [inline] unsafe fn from_raw_fd (fd : RawFd) -> process :: Stdio { let fd = sys :: fd :: FileDesc :: from_raw_fd (fd) ; let io = sys :: process :: Stdio :: Fd (fd) ; process :: Stdio :: from_inner (io) } }
};
}
