// Generated macro for impl_1966 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1966 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1966"}
// Dependencies: {}
# [stable (feature = "as_windows_ptrs" , since = "1.71.0")] impl < T : AsSocket > AsSocket for Box < T > { # [inline] fn as_socket (& self) -> BorrowedSocket < '_ > { (* * self) . as_socket () } }
};
}
