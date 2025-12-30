// Generated macro for impl_1889 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1889 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1889"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsHandle for crate :: process :: ChildStdout { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { unsafe { BorrowedHandle :: borrow_raw (self . as_raw_handle ()) } } }
};
}
