// Generated macro for impl_1961 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1961 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1961"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < T : AsSocket > AsSocket for & T { # [inline] fn as_socket (& self) -> BorrowedSocket < '_ > { T :: as_socket (self) } }
};
}
