// Generated macro for impl_2707 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2707 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2707"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] # [cfg (not (target_os = "trusty"))] impl From < OwnedFd > for io :: PipeWriter { fn from (owned_fd : OwnedFd) -> Self { Self (FromInner :: from_inner (owned_fd)) } }
};
}
