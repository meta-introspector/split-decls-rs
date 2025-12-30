// Generated macro for impl_2643 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2643 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2643"}
// Dependencies: {}
# [stable (feature = "asfd_rc" , since = "1.69.0")] impl < T : AsRawFd > AsRawFd for crate :: rc :: Rc < T > { # [inline] fn as_raw_fd (& self) -> RawFd { (* * self) . as_raw_fd () } }
};
}
