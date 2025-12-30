// Generated macro for impl_1965 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1965 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1965"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : AsSocket + ? Sized > AsSocket for crate :: rc :: UniqueRc < T > { # [inline] fn as_socket (& self) -> BorrowedSocket < '_ > { (* * self) . as_socket () } }
};
}
