// Generated macro for impl_2681 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2681 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2681"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] # [cfg (not (target_os = "trusty"))] impl From < fs :: File > for OwnedFd { # [doc = " Takes ownership of a [`File`](fs::File)'s underlying file descriptor."] # [inline] fn from (file : fs :: File) -> OwnedFd { file . into_inner () . into_inner () . into_inner () } }
};
}
