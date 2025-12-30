// Generated macro for impl_1563 (impl)
macro_rules! Depcrate_os_unix_net_listenerimpl_1563 {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"impl_1563"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < UnixListener > for OwnedFd { # [doc = " Takes ownership of a [`UnixListener`]'s socket file descriptor."] # [inline] fn from (listener : UnixListener) -> OwnedFd { listener . 0 . into_inner () . into_inner () } }
};
}
