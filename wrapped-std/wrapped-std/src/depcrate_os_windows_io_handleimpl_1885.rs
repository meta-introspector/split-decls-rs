// Generated macro for impl_1885 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1885 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1885"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsHandle for crate :: io :: Stderr { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { unsafe { BorrowedHandle :: borrow_raw (self . as_raw_handle ()) } } }
};
}
