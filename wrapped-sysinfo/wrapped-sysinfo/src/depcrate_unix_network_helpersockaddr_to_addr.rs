// Generated macro for sockaddr_to_addr (function)
macro_rules! Depcrate_unix_network_helpersockaddr_to_addr {
() => {
// Module: crate::unix::network_helper
// Provides: {"sockaddr_to_addr"}
// Dependencies: {}
fn sockaddr_to_addr (storage : & libc :: sockaddr_storage , len : usize) -> io :: Result < SocketAddr > { match storage . ss_family as libc :: c_int { libc :: AF_INET => { assert ! (len >= mem :: size_of ::< libc :: sockaddr_in > ()) ; let storage : & libc :: sockaddr_in = unsafe { mem :: transmute (storage) } ; let ip = (storage . sin_addr . s_addr as InAddrType) . to_be () ; let a = (ip >> 24) as u8 ; let b = (ip >> 16) as u8 ; let c = (ip >> 8) as u8 ; let d = ip as u8 ; let sockaddrv4 = SocketAddrV4 :: new (Ipv4Addr :: new (a , b , c , d) , storage . sin_port . to_be ()) ; Ok (SocketAddr :: V4 (sockaddrv4)) } libc :: AF_INET6 => { assert ! (len >= mem :: size_of ::< libc :: sockaddr_in6 > ()) ; let storage : & libc :: sockaddr_in6 = unsafe { mem :: transmute (storage) } ; let arr : [u16 ; 8] = unsafe { mem :: transmute (storage . sin6_addr . s6_addr) } ; let ip = Ipv6Addr :: new (arr [0] . to_be () , arr [1] . to_be () , arr [2] . to_be () , arr [3] . to_be () , arr [4] . to_be () , arr [5] . to_be () , arr [6] . to_be () , arr [7] . to_be () ,) ; Ok (SocketAddr :: V6 (SocketAddrV6 :: new (ip , storage . sin6_port . to_be () , u32 :: from_be (storage . sin6_flowinfo) , storage . sin6_scope_id ,))) } _ => Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "expected IPv4 or IPv6 socket" ,)) , } }
};
}
