// Generated macro for impl_2680 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2680 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2680"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] # [cfg (not (target_os = "trusty"))] impl AsFd for fs :: File { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { self . as_inner () . as_fd () } }
};
}
