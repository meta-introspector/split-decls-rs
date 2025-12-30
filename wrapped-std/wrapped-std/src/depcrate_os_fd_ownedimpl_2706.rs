// Generated macro for impl_2706 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2706 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2706"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] # [cfg (not (target_os = "trusty"))] impl From < OwnedFd > for io :: PipeReader { fn from (owned_fd : OwnedFd) -> Self { Self (FromInner :: from_inner (owned_fd)) } }
};
}
