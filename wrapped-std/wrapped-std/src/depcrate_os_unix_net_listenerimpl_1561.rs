// Generated macro for impl_1561 (impl)
macro_rules! Depcrate_os_unix_net_listenerimpl_1561 {
() => {
// Module: crate::os::unix::net::listener
// Provides: {"impl_1561"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsFd for UnixListener { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { self . 0 . as_inner () . as_fd () } }
};
}
