// Generated macro for send_vectored_with_ancillary_to (function)
macro_rules! Depcrate_os_unix_net_ancillarysend_vectored_with_ancillary_to {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"send_vectored_with_ancillary_to"}
// Dependencies: {}
pub (super) fn send_vectored_with_ancillary_to (socket : & Socket , path : Option < & Path > , bufs : & [IoSlice < '_ >] , ancillary : & mut SocketAncillary < '_ > ,) -> io :: Result < usize > { unsafe { let (mut msg_name , msg_namelen) = if let Some (path) = path { sockaddr_un (path) ? } else { (zeroed () , 0) } ; let mut msg : libc :: msghdr = zeroed () ; msg . msg_name = (& raw mut msg_name) as * mut _ ; msg . msg_namelen = msg_namelen ; msg . msg_iov = bufs . as_ptr () as * mut _ ; msg . msg_iovlen = bufs . len () as _ ; msg . msg_controllen = ancillary . length as _ ; if msg . msg_controllen > 0 { msg . msg_control = ancillary . buffer . as_mut_ptr () . cast () ; } ancillary . truncated = false ; socket . send_msg (& mut msg) } }
};
}
