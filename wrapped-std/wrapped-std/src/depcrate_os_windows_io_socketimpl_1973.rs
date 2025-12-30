// Generated macro for impl_1973 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1973 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1973"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < crate :: net :: TcpListener > for OwnedSocket { # [doc = " Takes ownership of a [`TcpListener`](crate::net::TcpListener)'s socket."] # [inline] fn from (tcp_listener : crate :: net :: TcpListener) -> OwnedSocket { unsafe { OwnedSocket :: from_raw_socket (tcp_listener . into_raw_socket ()) } } }
};
}
