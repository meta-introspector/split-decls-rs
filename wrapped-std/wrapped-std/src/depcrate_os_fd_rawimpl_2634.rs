// Generated macro for impl_2634 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2634 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2634"}
// Dependencies: {}
# [stable (feature = "from_raw_os" , since = "1.1.0")] # [cfg (not (target_os = "trusty"))] impl FromRawFd for fs :: File { # [inline] unsafe fn from_raw_fd (fd : RawFd) -> fs :: File { unsafe { fs :: File :: from (OwnedFd :: from_raw_fd (fd)) } } }
};
}
