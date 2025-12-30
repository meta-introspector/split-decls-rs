// Generated macro for impl_2644 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2644 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2644"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : AsRawFd + ? Sized > AsRawFd for crate :: rc :: UniqueRc < T > { # [inline] fn as_raw_fd (& self) -> RawFd { (* * self) . as_raw_fd () } }
};
}
