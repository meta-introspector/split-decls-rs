// Generated macro for impl_1298 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1298 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1298"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for (Ipv6Addr , u16) { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { let (ip , port) = * self ; SocketAddrV6 :: new (ip , port , 0 , 0) . to_socket_addrs () } }
};
}
