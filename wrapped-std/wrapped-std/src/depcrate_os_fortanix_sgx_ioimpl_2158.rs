// Generated macro for impl_2158 (impl)
macro_rules! Depcrate_os_fortanix_sgx_ioimpl_2158 {
() => {
// Module: crate::os::fortanix_sgx::io
// Provides: {"impl_2158"}
// Dependencies: {}
impl TryIntoRawFd for net :: TcpStream { # [inline] fn try_into_raw_fd (self) -> Result < RawFd , Self > { let (socket , peer_addr) = self . into_inner () . into_inner () ; match socket . try_into_inner () { Ok (fd) => Ok (fd . into_inner ()) , Err (socket) => { let sys = sys :: net :: TcpStream :: from_inner ((socket , peer_addr)) ; Err (net :: TcpStream :: from_inner (sys)) } } } }
};
}
