// Generated macro for impl_2635 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2635 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2635"}
// Dependencies: {}
# [stable (feature = "into_raw_os" , since = "1.4.0")] # [cfg (not (target_os = "trusty"))] impl IntoRawFd for fs :: File { # [inline] fn into_raw_fd (self) -> RawFd { self . into_inner () . into_inner () . into_raw_fd () } }
};
}
