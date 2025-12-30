// Generated macro for sockaddr_to_network_addr (function)
macro_rules! Depcrate_unix_network_helpersockaddr_to_network_addr {
() => {
// Module: crate::unix::network_helper
// Provides: {"sockaddr_to_network_addr"}
// Dependencies: {}
# [cfg (any (target_os = "openbsd" , target_os = "freebsd" , target_os = "netbsd" , target_os = "illumos" , target_os = "solaris" , target_os = "macos" , target_os = "ios"))] fn sockaddr_to_network_addr (sa : * const libc :: sockaddr) -> Option < IpAddr > { unsafe { if sa . is_null () || (* sa) . sa_family as libc :: c_int == 18 { None } else { let addr = sockaddr_to_addr (& (sa as * const libc :: sockaddr_storage) . read_unaligned () , mem :: size_of :: < libc :: sockaddr_storage > () ,) ; match addr { Ok (SocketAddr :: V4 (sa)) => Some (IpAddr :: V4 (* sa . ip ())) , Ok (SocketAddr :: V6 (sa)) => Some (IpAddr :: V6 (* sa . ip ())) , _ => None , } } } }
};
}
