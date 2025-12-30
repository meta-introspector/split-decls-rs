// Generated macro for impl_2666 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2666 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2666"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsRawFd for BorrowedFd < '_ > { # [inline] fn as_raw_fd (& self) -> RawFd { self . fd . as_inner () } }
};
}
