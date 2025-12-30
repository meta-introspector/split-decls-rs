// Generated macro for impl_2155 (impl)
macro_rules! Depcrate_os_fortanix_sgx_ioimpl_2155 {
() => {
// Module: crate::os::fortanix_sgx::io
// Provides: {"impl_2155"}
// Dependencies: {}
impl FromRawFd for net :: TcpStream { type Metadata = TcpStreamMetadata ; # [inline] unsafe fn from_raw_fd (fd : RawFd , metadata : Self :: Metadata) -> net :: TcpStream { let fd = sys :: fd :: FileDesc :: from_inner (fd) ; let socket = sys :: net :: Socket :: from_inner ((fd , metadata . local_addr)) ; net :: TcpStream :: from_inner (sys :: net :: TcpStream :: from_inner ((socket , metadata . peer_addr))) } }
};
}
