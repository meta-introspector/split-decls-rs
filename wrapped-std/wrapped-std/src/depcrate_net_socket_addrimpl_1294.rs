// Generated macro for impl_1294 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1294 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1294"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for SocketAddrV4 { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { SocketAddr :: V4 (* self) . to_socket_addrs () } }
};
}
