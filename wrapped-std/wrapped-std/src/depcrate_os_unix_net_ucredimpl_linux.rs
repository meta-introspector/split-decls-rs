// Generated macro for impl_linux (module)
macro_rules! Depcrate_os_unix_net_ucredimpl_linux {
() => {
// Module: crate::os::unix::net::ucred
// Provides: {"impl_linux"}
// Dependencies: {}
# [cfg (any (target_os = "linux" , target_os = "android" , target_os = "cygwin"))] mod impl_linux { use libc :: { SO_PEERCRED , SOL_SOCKET , c_void , getsockopt , socklen_t , ucred } ; use super :: UCred ; use crate :: io ; use crate :: os :: unix :: io :: AsRawFd ; use crate :: os :: unix :: net :: UnixStream ; pub fn peer_cred (socket : & UnixStream) -> io :: Result < UCred > { let ucred_size = size_of :: < ucred > () ; assert ! (size_of ::< u32 > () <= size_of ::< usize > ()) ; assert ! (ucred_size <= u32 :: MAX as usize) ; let mut ucred_size = ucred_size as socklen_t ; let mut ucred : ucred = ucred { pid : 1 , uid : 1 , gid : 1 } ; unsafe { let ret = getsockopt (socket . as_raw_fd () , SOL_SOCKET , SO_PEERCRED , (& raw mut ucred) as * mut c_void , & mut ucred_size ,) ; if ret == 0 && ucred_size as usize == size_of :: < ucred > () { Ok (UCred { uid : ucred . uid , gid : ucred . gid , pid : Some (ucred . pid) }) } else { Err (io :: Error :: last_os_error ()) } } } }
};
}
