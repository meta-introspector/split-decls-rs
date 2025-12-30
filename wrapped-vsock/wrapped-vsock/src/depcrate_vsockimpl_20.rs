// Generated macro for impl_20 (impl)
macro_rules! Depcrate_vsockimpl_20 {
() => {
// Module: crate::vsock
// Provides: {"impl_20"}
// Dependencies: {}
impl VsockListener { # [doc = " Create a new VsockListener which is bound and listening on the socket address."] pub fn bind (port : u32) -> io :: Result < Self > { unsafe { let saddr = sockaddr_vm { # [cfg (target_os = "hermit")] svm_len : std :: mem :: size_of :: < sockaddr_vm > () . try_into () . unwrap () , svm_reserved1 : 0 , svm_family : AF_VSOCK . try_into () . unwrap () , svm_cid : VMADDR_CID_ANY , svm_port : port , svm_zero : [0 ; 4] , } ; let fd = socket (AF_VSOCK , SOCK_STREAM , 0) ; check (bind (fd , & saddr as * const _ as * const sockaddr , std :: mem :: size_of :: < sockaddr_vm > () . try_into () . unwrap () ,)) ? ; check (listen (fd , 128)) ? ; Ok (VsockListener { fd : OwnedFd :: from_raw_fd (fd) , }) } } # [doc = " Accept a new incoming connection from this listener."] pub fn accept (& self) -> io :: Result < (VsockStream , VsockAddr) > { let mut vsock_addr_len : socklen_t = size_of :: < sockaddr_vm > () . try_into () . unwrap () ; let mut vsock_addr = sockaddr_vm { # [cfg (target_os = "hermit")] svm_len : vsock_addr_len . try_into () . unwrap () , svm_reserved1 : 0 , svm_family : AF_VSOCK as sa_family_t , svm_cid : 0 , svm_port : 0 , svm_zero : [0 ; 4] , } ; let fd = unsafe { check (accept (self . fd . as_raw_fd () , & mut vsock_addr as * mut _ as * mut sockaddr , & mut vsock_addr_len as * mut u32 ,)) ? } ; Ok ((VsockStream :: new (fd) , VsockAddr (vsock_addr))) } }
};
}
