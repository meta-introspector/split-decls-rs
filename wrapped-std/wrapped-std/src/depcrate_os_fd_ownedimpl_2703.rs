// Generated macro for impl_2703 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2703 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2703"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] # [cfg (not (target_os = "trusty"))] impl From < io :: PipeReader > for OwnedFd { fn from (pipe : io :: PipeReader) -> Self { pipe . 0 . into_inner () } }
};
}
