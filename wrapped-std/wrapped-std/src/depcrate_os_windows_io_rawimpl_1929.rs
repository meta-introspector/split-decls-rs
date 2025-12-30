// Generated macro for impl_1929 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1929 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1929"}
// Dependencies: {}
# [stable (feature = "from_raw_os" , since = "1.1.0")] impl FromRawSocket for net :: TcpListener { # [inline] unsafe fn from_raw_socket (sock : RawSocket) -> net :: TcpListener { unsafe { let sock = sys :: net :: Socket :: from_inner (OwnedSocket :: from_raw_socket (sock)) ; net :: TcpListener :: from_inner (sys :: net :: TcpListener :: from_inner (sock)) } } }
};
}
