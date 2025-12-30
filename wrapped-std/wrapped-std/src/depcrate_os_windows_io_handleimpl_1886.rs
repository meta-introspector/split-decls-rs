// Generated macro for impl_1886 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1886 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1886"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < 'a > AsHandle for crate :: io :: StderrLock < 'a > { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { unsafe { BorrowedHandle :: borrow_raw (self . as_raw_handle ()) } } }
};
}
