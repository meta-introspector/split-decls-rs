// Generated macro for impl_2685 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2685 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2685"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] # [cfg (not (target_os = "trusty"))] impl From < OwnedFd > for crate :: net :: TcpStream { # [inline] fn from (owned_fd : OwnedFd) -> Self { Self :: from_inner (FromInner :: from_inner (FromInner :: from_inner (FromInner :: from_inner (owned_fd ,)))) } }
};
}
