// Generated macro for impl_12 (impl)
macro_rules! Depcrate_vsockimpl_12 {
() => {
// Module: crate::vsock
// Provides: {"impl_12"}
// Dependencies: {}
impl VsockAddr { pub fn new (cid : u32 , port : u32) -> Self { # [cfg (target_os = "hermit")] let vsock_addr_len : socklen_t = size_of :: < sockaddr_vm > () . try_into () . unwrap () ; let vsock_addr = sockaddr_vm { # [cfg (target_os = "hermit")] svm_len : vsock_addr_len . try_into () . unwrap () , svm_reserved1 : 0 , svm_family : AF_VSOCK as sa_family_t , svm_cid : cid , svm_port : port , svm_zero : [0 ; 4] , } ; Self (vsock_addr) } }
};
}
