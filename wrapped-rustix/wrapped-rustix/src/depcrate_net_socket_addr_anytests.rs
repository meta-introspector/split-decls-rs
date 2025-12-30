// Generated macro for tests (module)
macro_rules! Depcrate_net_socket_addr_anytests {
() => {
// Module: crate::net::socket_addr_any
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn any_read () { let localhost = std :: net :: Ipv6Addr :: LOCALHOST ; let addr = SocketAddrAny :: from (SocketAddrV6 :: new (localhost , 7 , 8 , 9)) ; unsafe { let same = SocketAddrAny :: read (addr . as_ptr () , addr . addr_len ()) ; assert_eq ! (addr , same) ; } } }
};
}
