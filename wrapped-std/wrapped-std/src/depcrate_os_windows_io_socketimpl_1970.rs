// Generated macro for impl_1970 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1970 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1970"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < crate :: net :: TcpStream > for OwnedSocket { # [doc = " Takes ownership of a [`TcpStream`](crate::net::TcpStream)'s socket."] # [inline] fn from (tcp_stream : crate :: net :: TcpStream) -> OwnedSocket { unsafe { OwnedSocket :: from_raw_socket (tcp_stream . into_raw_socket ()) } } }
};
}
