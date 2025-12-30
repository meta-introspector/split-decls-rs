// Generated macro for impl_2677 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2677 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2677"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < T : AsFd + ? Sized > AsFd for & mut T { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { T :: as_fd (self) } }
};
}
