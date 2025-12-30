// Generated macro for AsSocket (trait)
macro_rules! Depcrate_os_windows_io_socketAsSocket {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"AsSocket"}
// Dependencies: {}
# [doc = " A trait to borrow the socket from an underlying object."] # [stable (feature = "io_safety" , since = "1.63.0")] pub trait AsSocket { # [doc = " Borrows the socket."] # [stable (feature = "io_safety" , since = "1.63.0")] fn as_socket (& self) -> BorrowedSocket < '_ > ; }
};
}
