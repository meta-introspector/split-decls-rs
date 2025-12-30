// Generated macro for impl_2633 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2633 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2633"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] # [cfg (not (target_os = "trusty"))] impl AsRawFd for fs :: File { # [inline] fn as_raw_fd (& self) -> RawFd { self . as_inner () . as_raw_fd () } }
};
}
