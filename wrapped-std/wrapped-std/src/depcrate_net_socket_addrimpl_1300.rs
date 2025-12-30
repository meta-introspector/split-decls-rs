// Generated macro for impl_1300 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1300 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1300"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for (& str , u16) { type Iter = vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < vec :: IntoIter < SocketAddr > > { let (host , port) = * self ; if let Ok (addr) = host . parse :: < Ipv4Addr > () { let addr = SocketAddrV4 :: new (addr , port) ; return Ok (vec ! [SocketAddr :: V4 (addr)] . into_iter ()) ; } if let Ok (addr) = host . parse :: < Ipv6Addr > () { let addr = SocketAddrV6 :: new (addr , port , 0 , 0) ; return Ok (vec ! [SocketAddr :: V6 (addr)] . into_iter ()) ; } resolve_socket_addr ((host , port) . try_into () ?) } }
};
}
