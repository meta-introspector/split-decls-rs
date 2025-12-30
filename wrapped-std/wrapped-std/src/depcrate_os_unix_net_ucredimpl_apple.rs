// Generated macro for impl_apple (module)
macro_rules! Depcrate_os_unix_net_ucredimpl_apple {
() => {
// Module: crate::os::unix::net::ucred
// Provides: {"impl_apple"}
// Dependencies: {}
# [cfg (target_vendor = "apple")] mod impl_apple { use libc :: { LOCAL_PEERPID , SOL_LOCAL , c_void , getpeereid , getsockopt , pid_t , socklen_t } ; use super :: UCred ; use crate :: io ; use crate :: os :: unix :: io :: AsRawFd ; use crate :: os :: unix :: net :: UnixStream ; pub fn peer_cred (socket : & UnixStream) -> io :: Result < UCred > { let mut cred = UCred { uid : 1 , gid : 1 , pid : None } ; unsafe { let ret = getpeereid (socket . as_raw_fd () , & mut cred . uid , & mut cred . gid) ; if ret != 0 { return Err (io :: Error :: last_os_error ()) ; } let mut pid : pid_t = 1 ; let mut pid_size = size_of :: < pid_t > () as socklen_t ; let ret = getsockopt (socket . as_raw_fd () , SOL_LOCAL , LOCAL_PEERPID , (& raw mut pid) as * mut c_void , & mut pid_size ,) ; if ret == 0 && pid_size as usize == size_of :: < pid_t > () { cred . pid = Some (pid) ; Ok (cred) } else { Err (io :: Error :: last_os_error ()) } } } }
};
}
