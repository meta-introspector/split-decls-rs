// Generated macro for impl_1957 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1957 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1957"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl Drop for OwnedSocket { # [inline] fn drop (& mut self) { unsafe { let _ = sys :: c :: closesocket (self . socket . as_inner () as sys :: c :: SOCKET) ; } } }
};
}
