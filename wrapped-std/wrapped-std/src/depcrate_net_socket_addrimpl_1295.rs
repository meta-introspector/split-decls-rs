// Generated macro for impl_1295 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1295 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1295"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for SocketAddrV6 { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { SocketAddr :: V6 (* self) . to_socket_addrs () } }
};
}
