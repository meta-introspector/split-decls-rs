// Generated macro for impl_1293 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1293 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1293"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for SocketAddr { type Iter = option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < option :: IntoIter < SocketAddr > > { Ok (Some (* self) . into_iter ()) } }
};
}
