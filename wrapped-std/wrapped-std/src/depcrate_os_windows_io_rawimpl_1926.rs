// Generated macro for impl_1926 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1926 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1926"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl AsRawSocket for net :: TcpListener { # [inline] fn as_raw_socket (& self) -> RawSocket { self . as_inner () . socket () . as_raw_socket () } }
};
}
