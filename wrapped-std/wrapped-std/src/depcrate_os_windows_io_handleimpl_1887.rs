// Generated macro for impl_1887 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1887 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1887"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsHandle for crate :: process :: ChildStdin { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { unsafe { BorrowedHandle :: borrow_raw (self . as_raw_handle ()) } } }
};
}
