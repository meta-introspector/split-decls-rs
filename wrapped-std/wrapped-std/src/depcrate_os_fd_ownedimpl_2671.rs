// Generated macro for impl_2671 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2671 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2671"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl fmt :: Debug for BorrowedFd < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BorrowedFd") . field ("fd" , & self . fd) . finish () } }
};
}
