// Generated macro for impl_1968 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1968 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1968"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsSocket for OwnedSocket { # [inline] fn as_socket (& self) -> BorrowedSocket < '_ > { unsafe { BorrowedSocket :: borrow_raw (self . as_raw_socket ()) } } }
};
}
