// Generated macro for impl_1971 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1971 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1971"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < OwnedSocket > for crate :: net :: TcpStream { # [inline] fn from (owned : OwnedSocket) -> Self { unsafe { Self :: from_raw_socket (owned . into_raw_socket ()) } } }
};
}
