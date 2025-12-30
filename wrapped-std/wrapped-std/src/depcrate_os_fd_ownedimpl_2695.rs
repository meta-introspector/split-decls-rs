// Generated macro for impl_2695 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2695 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2695"}
// Dependencies: {}
# [stable (feature = "asfd_ptrs" , since = "1.64.0")] impl < T : AsFd + ? Sized > AsFd for Box < T > { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { (* * self) . as_fd () } }
};
}
