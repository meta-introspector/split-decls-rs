// Generated macro for impl_1562 (impl)
macro_rules! Depcrate_os_unix_net_listenerimpl_1562 {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"impl_1562"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < OwnedFd > for UnixListener { # [inline] fn from (fd : OwnedFd) -> UnixListener { UnixListener (Socket :: from_inner (FromInner :: from_inner (fd))) } }
};
}
