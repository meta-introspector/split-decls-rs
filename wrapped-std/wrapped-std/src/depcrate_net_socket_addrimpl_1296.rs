// Generated macro for impl_1296 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1296 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1296"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for (IpAddr , u16) { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { let (ip , port) = * self ; match ip { IpAddr :: V4 (ref a) => (* a , port) . to_socket_addrs () , IpAddr :: V6 (ref a) => (* a , port) . to_socket_addrs () , } } }
};
}
