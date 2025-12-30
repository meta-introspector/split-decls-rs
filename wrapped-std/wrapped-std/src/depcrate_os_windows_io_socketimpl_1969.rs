// Generated macro for impl_1969 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1969 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1969"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsSocket for crate :: net :: TcpStream { # [inline] fn as_socket (& self) -> BorrowedSocket < '_ > { unsafe { BorrowedSocket :: borrow_raw (self . as_raw_socket ()) } } }
};
}
