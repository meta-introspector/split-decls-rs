// Generated macro for impl_1891 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1891 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1891"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsHandle for crate :: process :: ChildStderr { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { unsafe { BorrowedHandle :: borrow_raw (self . as_raw_handle ()) } } }
};
}
