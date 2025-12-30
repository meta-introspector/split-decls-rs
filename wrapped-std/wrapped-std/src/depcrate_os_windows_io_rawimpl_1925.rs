// Generated macro for impl_1925 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1925 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1925"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl AsRawSocket for net :: TcpStream { # [inline] fn as_raw_socket (& self) -> RawSocket { self . as_inner () . socket () . as_raw_socket () } }
};
}
