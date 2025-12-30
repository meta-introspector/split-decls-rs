// Generated macro for impl_2676 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2676 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2676"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < T : AsFd + ? Sized > AsFd for & T { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { T :: as_fd (self) } }
};
}
