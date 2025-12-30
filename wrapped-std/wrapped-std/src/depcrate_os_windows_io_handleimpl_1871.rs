// Generated macro for impl_1871 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1871 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1871"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < T : AsHandle + ? Sized > AsHandle for & mut T { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { T :: as_handle (self) } }
};
}
