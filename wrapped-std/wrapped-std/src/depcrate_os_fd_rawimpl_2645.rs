// Generated macro for impl_2645 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2645 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2645"}
// Dependencies: {}
# [stable (feature = "asrawfd_ptrs" , since = "1.63.0")] impl < T : AsRawFd > AsRawFd for Box < T > { # [inline] fn as_raw_fd (& self) -> RawFd { (* * self) . as_raw_fd () } }
};
}
