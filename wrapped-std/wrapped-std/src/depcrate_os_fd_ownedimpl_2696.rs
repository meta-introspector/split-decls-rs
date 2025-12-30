// Generated macro for impl_2696 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2696 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2696"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsFd for io :: Stdin { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { unsafe { BorrowedFd :: borrow_raw (0) } } }
};
}
