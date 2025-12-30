// Generated macro for impl_1302 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1302 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1302"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ToSocketAddrs for str { type Iter = vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> io :: Result < vec :: IntoIter < SocketAddr > > { if let Ok (addr) = self . parse () { return Ok (vec ! [addr] . into_iter ()) ; } resolve_socket_addr (self . try_into () ?) } }
};
}
