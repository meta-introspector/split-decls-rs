// Generated macro for impl_2704 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2704 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2704"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] # [cfg (not (target_os = "trusty"))] impl AsFd for io :: PipeWriter { fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 . as_fd () } }
};
}
