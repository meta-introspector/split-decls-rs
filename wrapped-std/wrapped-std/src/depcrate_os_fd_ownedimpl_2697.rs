// Generated macro for impl_2697 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2697 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2697"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < 'a > AsFd for io :: StdinLock < 'a > { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { unsafe { BorrowedFd :: borrow_raw (0) } } }
};
}
