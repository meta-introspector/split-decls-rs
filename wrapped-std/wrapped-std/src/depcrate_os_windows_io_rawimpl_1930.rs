// Generated macro for impl_1930 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1930 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1930"}
// Dependencies: {}
# [stable (feature = "from_raw_os" , since = "1.1.0")] impl FromRawSocket for net :: UdpSocket { # [inline] unsafe fn from_raw_socket (sock : RawSocket) -> net :: UdpSocket { unsafe { let sock = sys :: net :: Socket :: from_inner (OwnedSocket :: from_raw_socket (sock)) ; net :: UdpSocket :: from_inner (sys :: net :: UdpSocket :: from_inner (sock)) } } }
};
}
