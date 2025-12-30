// Generated macro for TcpStreamMetadata (struct)
macro_rules! Depcrate_os_fortanix_sgx_ioTcpStreamMetadata {
() => {
// Module: crate::os::fortanix_sgx::io
// Provides: {"TcpStreamMetadata"}
// Dependencies: {}
# [doc = " Metadata for `TcpStream`."] # [derive (Debug , Clone , Default)] # [unstable (feature = "sgx_platform" , issue = "56975")] pub struct TcpStreamMetadata { # [doc = " Local address of the TCP stream"] pub local_addr : Option < String > , # [doc = " Peer address of the TCP stream"] pub peer_addr : Option < String > , }
};
}
