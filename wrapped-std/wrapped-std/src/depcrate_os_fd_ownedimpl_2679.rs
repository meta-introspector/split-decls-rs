// Generated macro for impl_2679 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2679 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2679"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsFd for OwnedFd { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { unsafe { BorrowedFd :: borrow_raw (self . as_raw_fd ()) } } }
};
}
