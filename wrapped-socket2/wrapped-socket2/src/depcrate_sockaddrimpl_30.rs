// Generated macro for impl_30 (impl)
macro_rules! Depcrate_sockaddrimpl_30 {
() => {
// Module: crate::sockaddr
// Provides: {"impl_30"}
// Dependencies: {}
impl From < SocketAddrV6 > for SockAddr { fn from (addr : SocketAddrV6) -> SockAddr { let mut storage = unsafe { mem :: zeroed :: < sockaddr_storage > () } ; let len = { let storage = unsafe { & mut * ptr :: addr_of_mut ! (storage) . cast :: < sockaddr_in6 > () } ; storage . sin6_family = AF_INET6 as sa_family_t ; storage . sin6_port = addr . port () . to_be () ; storage . sin6_addr = crate :: sys :: to_in6_addr (addr . ip ()) ; storage . sin6_flowinfo = addr . flowinfo () ; # [cfg (unix)] { storage . sin6_scope_id = addr . scope_id () ; } # [cfg (windows)] { storage . Anonymous = SOCKADDR_IN6_0 { sin6_scope_id : addr . scope_id () , } ; } mem :: size_of :: < sockaddr_in6 > () as socklen_t } ; # [cfg (any (target_os = "dragonfly" , target_os = "freebsd" , target_os = "haiku" , target_os = "hermit" , target_os = "ios" , target_os = "visionos" , target_os = "macos" , target_os = "netbsd" , target_os = "nto" , target_os = "openbsd" , target_os = "tvos" , target_os = "vxworks" , target_os = "watchos" ,))] { storage . ss_len = len as u8 ; } SockAddr { storage , len } } }
};
}
