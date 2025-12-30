// Generated macro for impl_1874 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1874 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1874"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : AsHandle + ? Sized > AsHandle for crate :: rc :: UniqueRc < T > { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { (* * self) . as_handle () } }
};
}
