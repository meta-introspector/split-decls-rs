// Generated macro for to_c_sockaddr (function)
macro_rules! Depcrate_network_reachabilityto_c_sockaddr {
() => {
// Module: crate::network_reachability
// Provides: {"to_c_sockaddr"}
// Dependencies: {}
# [doc = " Allocates a libc::sockaddr compatible struct and fills it with either a libc::sockaddr_in or a"] # [doc = " libc::sockaddr_in6, depending on the passed in standard library SocketAddr."] fn to_c_sockaddr (addr : SocketAddr) -> Box < libc :: sockaddr > { let ptr = match addr { SocketAddr :: V4 (addr) => Box :: into_raw (Box :: new (libc :: sockaddr_in { sin_len : std :: mem :: size_of :: < libc :: sockaddr_in > () as u8 , sin_family : libc :: AF_INET as libc :: sa_family_t , sin_port : addr . port () . to_be () , sin_addr : { libc :: in_addr { s_addr : u32 :: from_ne_bytes (addr . ip () . octets ()) , } } , sin_zero : Default :: default () , })) as * mut c_void , SocketAddr :: V6 (addr) => Box :: into_raw (Box :: new (libc :: sockaddr_in6 { sin6_len : std :: mem :: size_of :: < libc :: sockaddr_in6 > () as u8 , sin6_family : libc :: AF_INET6 as libc :: sa_family_t , sin6_port : addr . port () . to_be () , sin6_flowinfo : addr . flowinfo () , sin6_addr : libc :: in6_addr { s6_addr : addr . ip () . octets () , } , sin6_scope_id : addr . scope_id () , })) as * mut c_void , } ; unsafe { Box :: from_raw (ptr as * mut _) } }
};
}
