// Generated macro for impl_2698 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2698 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2698"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsFd for io :: Stdout { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { unsafe { BorrowedFd :: borrow_raw (1) } } }
};
}
