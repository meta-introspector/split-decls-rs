// Generated macro for impl_1877 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1877 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1877"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsHandle for OwnedHandle { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { unsafe { BorrowedHandle :: borrow_raw (self . as_raw_handle ()) } } }
};
}
