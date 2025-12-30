// Generated macro for recv_vectored_with_ancillary_from (function)
macro_rules! Depcrate_os_unix_net_ancillaryrecv_vectored_with_ancillary_from {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"recv_vectored_with_ancillary_from"}
// Dependencies: {}
pub (super) fn recv_vectored_with_ancillary_from (socket : & Socket , bufs : & mut [IoSliceMut < '_ >] , ancillary : & mut SocketAncillary < '_ > ,) -> io :: Result < (usize , bool , io :: Result < SocketAddr >) > { unsafe { let mut msg_name : libc :: sockaddr_un = zeroed () ; let mut msg : libc :: msghdr = zeroed () ; msg . msg_name = (& raw mut msg_name) as * mut _ ; msg . msg_namelen = size_of :: < libc :: sockaddr_un > () as libc :: socklen_t ; msg . msg_iov = bufs . as_mut_ptr () . cast () ; msg . msg_iovlen = bufs . len () as _ ; msg . msg_controllen = ancillary . buffer . len () as _ ; if msg . msg_controllen > 0 { msg . msg_control = ancillary . buffer . as_mut_ptr () . cast () ; } let count = socket . recv_msg (& mut msg) ? ; ancillary . length = msg . msg_controllen as usize ; ancillary . truncated = msg . msg_flags & libc :: MSG_CTRUNC == libc :: MSG_CTRUNC ; let truncated = msg . msg_flags & libc :: MSG_TRUNC == libc :: MSG_TRUNC ; let addr = SocketAddr :: from_parts (msg_name , msg . msg_namelen) ; Ok ((count , truncated , addr)) } }
};
}
