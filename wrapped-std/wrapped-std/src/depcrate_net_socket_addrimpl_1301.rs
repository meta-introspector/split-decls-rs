// Generated macro for impl_1301 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1301 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1301"}
// Dependencies: {}
# [stable (feature = "string_u16_to_socket_addrs" , since = "1.46.0")] impl ToSocketAddrs for (String , u16) { type Iter = vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < vec :: IntoIter < SocketAddr > > { (& * self . 0 , self . 1) . to_socket_addrs () } }
};
}
