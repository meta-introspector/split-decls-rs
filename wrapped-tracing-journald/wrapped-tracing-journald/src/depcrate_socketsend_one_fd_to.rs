// Generated macro for send_one_fd_to (function)
macro_rules! Depcrate_socketsend_one_fd_to {
() => {
// Module: crate::socket
// Provides: {"send_one_fd_to"}
// Dependencies: {}
pub fn send_one_fd_to < P : AsRef < Path > > (socket : & UnixDatagram , fd : RawFd , path : P) -> Result < usize > { assert_cmsg_bufsize () ; let mut addr : sockaddr_un = unsafe { zeroed () } ; let path_bytes = path . as_ref () . as_os_str () . as_bytes () ; if addr . sun_path . len () <= path_bytes . len () { return Err (Error :: from_raw_os_error (ENAMETOOLONG)) ; } addr . sun_family = AF_UNIX as _ ; unsafe { std :: ptr :: copy_nonoverlapping (path_bytes . as_ptr () , addr . sun_path . as_mut_ptr () as * mut u8 , path_bytes . len () ,) } ; let mut msg : msghdr = unsafe { zeroed () } ; msg . msg_name = & mut addr as * mut _ as * mut c_void ; msg . msg_namelen = size_of :: < sockaddr_un > () as socklen_t ; msg . msg_iov = ptr :: null_mut () ; msg . msg_iovlen = 0 ; let mut cmsg_buffer = AlignedBuffer { buffer : ([0u8 ; CMSG_BUFSIZE]) , } ; msg . msg_control = unsafe { cmsg_buffer . buffer . as_mut_ptr () as _ } ; msg . msg_controllen = unsafe { CMSG_SPACE (size_of :: < RawFd > () as _) as _ } ; let cmsg : & mut cmsghdr = unsafe { CMSG_FIRSTHDR (& msg) . as_mut () } . expect ("Control message buffer exhausted") ; cmsg . cmsg_level = SOL_SOCKET ; cmsg . cmsg_type = SCM_RIGHTS ; cmsg . cmsg_len = unsafe { CMSG_LEN (size_of :: < RawFd > () as _) as _ } ; unsafe { ptr :: write (CMSG_DATA (cmsg) as * mut RawFd , fd) } ; let result = unsafe { sendmsg (socket . as_raw_fd () , & msg , libc :: MSG_NOSIGNAL) } ; if result < 0 { Err (Error :: last_os_error ()) } else { Ok (result as usize) } }
};
}
