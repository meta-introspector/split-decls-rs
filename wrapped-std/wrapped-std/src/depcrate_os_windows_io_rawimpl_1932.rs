// Generated macro for impl_1932 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1932 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1932"}
// Dependencies: {}
# [stable (feature = "into_raw_os" , since = "1.4.0")] impl IntoRawSocket for net :: TcpListener { # [inline] fn into_raw_socket (self) -> RawSocket { self . into_inner () . into_socket () . into_inner () . into_raw_socket () } }
};
}
