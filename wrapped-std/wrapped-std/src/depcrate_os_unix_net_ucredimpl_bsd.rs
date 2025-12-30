// Generated macro for impl_bsd (module)
macro_rules! Depcrate_os_unix_net_ucredimpl_bsd {
() => {
// Module: crate::os::unix::net::ucred
// Provides: {"impl_bsd"}
// Dependencies: {}
# [cfg (any (target_os = "dragonfly" , target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "nto" ,))] mod impl_bsd { use super :: UCred ; use crate :: io ; use crate :: os :: unix :: io :: AsRawFd ; use crate :: os :: unix :: net :: UnixStream ; pub fn peer_cred (socket : & UnixStream) -> io :: Result < UCred > { let mut cred = UCred { uid : 1 , gid : 1 , pid : None } ; unsafe { let ret = libc :: getpeereid (socket . as_raw_fd () , & mut cred . uid , & mut cred . gid) ; if ret == 0 { Ok (cred) } else { Err (io :: Error :: last_os_error ()) } } } }
};
}
