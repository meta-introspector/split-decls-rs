// Generated macro for impl_1893 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1893 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1893"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < T > AsHandle for crate :: thread :: JoinHandle < T > { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { unsafe { BorrowedHandle :: borrow_raw (self . as_raw_handle ()) } } }
};
}
