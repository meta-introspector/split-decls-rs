// Generated macro for impl_2689 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2689 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2689"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] # [cfg (not (target_os = "trusty"))] impl AsFd for crate :: net :: UdpSocket { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { self . as_inner () . socket () . as_fd () } }
};
}
