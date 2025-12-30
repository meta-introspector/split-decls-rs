// Generated macro for impl_1882 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1882 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1882"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < 'a > AsHandle for crate :: io :: StdinLock < 'a > { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { unsafe { BorrowedHandle :: borrow_raw (self . as_raw_handle ()) } } }
};
}
