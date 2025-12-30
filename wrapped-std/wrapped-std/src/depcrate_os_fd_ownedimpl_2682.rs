// Generated macro for impl_2682 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2682 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2682"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] # [cfg (not (target_os = "trusty"))] impl From < OwnedFd > for fs :: File { # [doc = " Returns a [`File`](fs::File) that takes ownership of the given"] # [doc = " file descriptor."] # [inline] fn from (owned_fd : OwnedFd) -> Self { Self :: from_inner (FromInner :: from_inner (FromInner :: from_inner (owned_fd))) } }
};
}
