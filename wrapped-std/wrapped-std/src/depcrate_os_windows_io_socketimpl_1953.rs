// Generated macro for impl_1953 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1953 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1953"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsRawSocket for BorrowedSocket < '_ > { # [inline] fn as_raw_socket (& self) -> RawSocket { self . socket . as_inner () } }
};
}
