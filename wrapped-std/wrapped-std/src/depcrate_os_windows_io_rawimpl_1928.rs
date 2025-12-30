// Generated macro for impl_1928 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1928 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1928"}
// Dependencies: {}
# [stable (feature = "from_raw_os" , since = "1.1.0")] impl FromRawSocket for net :: TcpStream { # [inline] unsafe fn from_raw_socket (sock : RawSocket) -> net :: TcpStream { unsafe { let sock = sys :: net :: Socket :: from_inner (OwnedSocket :: from_raw_socket (sock)) ; net :: TcpStream :: from_inner (sys :: net :: TcpStream :: from_inner (sock)) } } }
};
}
