// Generated macro for impl_1955 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1955 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1955"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl IntoRawSocket for OwnedSocket { # [inline] fn into_raw_socket (self) -> RawSocket { ManuallyDrop :: new (self) . socket . as_inner () } }
};
}
