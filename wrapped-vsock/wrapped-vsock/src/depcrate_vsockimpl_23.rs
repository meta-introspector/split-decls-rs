// Generated macro for impl_23 (impl)
macro_rules! Depcrate_vsockimpl_23 {
() => {
// Module: crate::vsock
// Provides: {"impl_23"}
// Dependencies: {}
impl VsockStream { pub fn new (fd : RawFd) -> Self { Self { fd : unsafe { FromRawFd :: from_raw_fd (fd) } , } } pub fn connect (addr : VsockAddr) -> io :: Result < VsockStream > { let len : socklen_t = size_of :: < sockaddr_vm > () . try_into () . unwrap () ; let fd = unsafe { socket (AF_VSOCK , SOCK_STREAM , 0) } ; unsafe { check (connect (fd . as_raw_fd () , & addr . 0 as * const _ as * const sockaddr , len ,)) ? } ; Ok (VsockStream :: new (fd)) } }
};
}
