// Generated macro for sockaddr_un (function)
macro_rules! Depcrate_os_unix_net_addrsockaddr_un {
() => {
// Module: crate::os::unix::net::addr
// Provides: {"sockaddr_un"}
// Dependencies: {}
pub (super) fn sockaddr_un (path : & Path) -> io :: Result < (libc :: sockaddr_un , libc :: socklen_t) > { let mut addr : libc :: sockaddr_un = unsafe { mem :: zeroed () } ; addr . sun_family = libc :: AF_UNIX as libc :: sa_family_t ; let bytes = path . as_os_str () . as_bytes () ; if bytes . contains (& 0) { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "paths must not contain interior null bytes" ,)) ; } if bytes . len () >= addr . sun_path . len () { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "path must be shorter than SUN_LEN" ,)) ; } unsafe { ptr :: copy_nonoverlapping (bytes . as_ptr () , addr . sun_path . as_mut_ptr () . cast () , bytes . len ()) } ; let mut len = SUN_PATH_OFFSET + bytes . len () ; match bytes . get (0) { Some (& 0) | None => { } Some (_) => len += 1 , } Ok ((addr , len as libc :: socklen_t)) }
};
}
