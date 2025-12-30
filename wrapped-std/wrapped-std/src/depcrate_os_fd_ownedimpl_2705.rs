// Generated macro for impl_2705 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2705 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2705"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] # [cfg (not (target_os = "trusty"))] impl From < io :: PipeWriter > for OwnedFd { fn from (pipe : io :: PipeWriter) -> Self { pipe . 0 . into_inner () } }
};
}
