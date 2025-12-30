// Generated macro for impl_1594 (impl)
macro_rules! Depcrate_os_unix_net_streamimpl_1594 {
() => {
// Module: crate::os::unix::net::stream
// Provides: {"impl_1594"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < UnixStream > for OwnedFd { # [doc = " Takes ownership of a [`UnixStream`]'s socket file descriptor."] # [inline] fn from (unix_stream : UnixStream) -> OwnedFd { unsafe { OwnedFd :: from_raw_fd (unix_stream . into_raw_fd ()) } } }
};
}
