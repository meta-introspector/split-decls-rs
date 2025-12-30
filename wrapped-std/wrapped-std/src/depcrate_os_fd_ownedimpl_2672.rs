// Generated macro for impl_2672 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2672 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2672"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl fmt :: Debug for OwnedFd { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OwnedFd") . field ("fd" , & self . fd) . finish () } }
};
}
