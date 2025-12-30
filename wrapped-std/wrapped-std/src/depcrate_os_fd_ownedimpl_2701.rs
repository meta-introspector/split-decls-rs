// Generated macro for impl_2701 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2701 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2701"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < 'a > AsFd for io :: StderrLock < 'a > { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { unsafe { BorrowedFd :: borrow_raw (2) } } }
};
}
