// Generated macro for impl_2157 (impl)
macro_rules! Depcrate_os_fortanix_sgx_ioimpl_2157 {
() => {
// Module: crate::os::fortanix_sgx::io
// Provides: {"impl_2157"}
// Dependencies: {}
impl FromRawFd for net :: TcpListener { type Metadata = TcpListenerMetadata ; # [inline] unsafe fn from_raw_fd (fd : RawFd , metadata : Self :: Metadata) -> net :: TcpListener { let fd = sys :: fd :: FileDesc :: from_inner (fd) ; let socket = sys :: net :: Socket :: from_inner ((fd , metadata . local_addr)) ; net :: TcpListener :: from_inner (sys :: net :: TcpListener :: from_inner (socket)) } }
};
}
