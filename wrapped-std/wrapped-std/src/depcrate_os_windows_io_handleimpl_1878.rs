// Generated macro for impl_1878 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1878 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1878"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsHandle for fs :: File { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { self . as_inner () . as_handle () } }
};
}
