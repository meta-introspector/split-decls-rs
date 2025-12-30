// Generated macro for impl_1303 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1303 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1303"}
// Dependencies: {}
# [stable (feature = "slice_to_socket_addrs" , since = "1.8.0")] impl < 'a > ToSocketAddrs for & 'a [SocketAddr] { type Iter = iter :: Cloned < slice :: Iter < 'a , SocketAddr > > ; fn to_socket_addrs (& self) -> io :: Result < Self :: Iter > { Ok (self . iter () . cloned ()) } }
};
}
