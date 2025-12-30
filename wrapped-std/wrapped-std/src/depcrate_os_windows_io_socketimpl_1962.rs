// Generated macro for impl_1962 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1962 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1962"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < T : AsSocket > AsSocket for & mut T { # [inline] fn as_socket (& self) -> BorrowedSocket < '_ > { T :: as_socket (self) } }
};
}
