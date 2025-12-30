// Generated macro for impl_2699 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2699 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2699"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < 'a > AsFd for io :: StdoutLock < 'a > { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { unsafe { BorrowedFd :: borrow_raw (1) } } }
};
}
