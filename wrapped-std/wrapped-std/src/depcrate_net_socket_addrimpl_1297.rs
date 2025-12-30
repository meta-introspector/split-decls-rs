// Generated macro for impl_1297 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1297 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1297"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for (Ipv4Addr , u16) { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { let (ip , port) = * self ; SocketAddrV4 :: new (ip , port) . to_socket_addrs () } }
};
}
