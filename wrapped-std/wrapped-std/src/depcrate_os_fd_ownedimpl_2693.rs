// Generated macro for impl_2693 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2693 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2693"}
// Dependencies: {}
# [stable (feature = "asfd_rc" , since = "1.69.0")] impl < T : AsFd + ? Sized > AsFd for crate :: rc :: Rc < T > { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { (* * self) . as_fd () } }
};
}
