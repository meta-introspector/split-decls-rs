// Generated macro for impl_1873 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1873 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1873"}
// Dependencies: {}
# [stable (feature = "as_windows_ptrs" , since = "1.71.0")] impl < T : AsHandle + ? Sized > AsHandle for crate :: rc :: Rc < T > { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { (* * self) . as_handle () } }
};
}
