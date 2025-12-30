// Generated macro for impl_2159 (impl)
macro_rules! Depcrate_os_fortanix_sgx_ioimpl_2159 {
() => {
// Module: crate::os::fortanix_sgx::io
// Provides: {"impl_2159"}
// Dependencies: {}
impl TryIntoRawFd for net :: TcpListener { # [inline] fn try_into_raw_fd (self) -> Result < RawFd , Self > { match self . into_inner () . into_inner () . try_into_inner () { Ok (fd) => Ok (fd . into_inner ()) , Err (socket) => { let sys = sys :: net :: TcpListener :: from_inner (socket) ; Err (net :: TcpListener :: from_inner (sys)) } } } }
};
}
