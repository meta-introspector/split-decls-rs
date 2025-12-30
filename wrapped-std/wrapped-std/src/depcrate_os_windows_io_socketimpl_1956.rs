// Generated macro for impl_1956 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1956 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1956"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl FromRawSocket for OwnedSocket { # [inline] # [track_caller] unsafe fn from_raw_socket (socket : RawSocket) -> Self { Self { socket : ValidRawSocket :: new (socket) . expect ("socket != -1") } } }
};
}
