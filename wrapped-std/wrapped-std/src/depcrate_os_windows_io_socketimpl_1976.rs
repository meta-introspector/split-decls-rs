// Generated macro for impl_1976 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1976 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1976"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < crate :: net :: UdpSocket > for OwnedSocket { # [doc = " Takes ownership of a [`UdpSocket`](crate::net::UdpSocket)'s underlying socket."] # [inline] fn from (udp_socket : crate :: net :: UdpSocket) -> OwnedSocket { unsafe { OwnedSocket :: from_raw_socket (udp_socket . into_raw_socket ()) } } }
};
}
