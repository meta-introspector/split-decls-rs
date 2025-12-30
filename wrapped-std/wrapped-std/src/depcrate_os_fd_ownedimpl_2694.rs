// Generated macro for impl_2694 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2694 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2694"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : AsFd + ? Sized > AsFd for crate :: rc :: UniqueRc < T > { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { (* * self) . as_fd () } }
};
}
