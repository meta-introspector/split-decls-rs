// Generated macro for impl_1304 (impl)
macro_rules! Depcrate_net_socket_addrimpl_1304 {
() => {
// Module: crate::net::socket_addr
// Provides: {"impl_1304"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ToSocketAddrs + ? Sized > ToSocketAddrs for & T { type Iter = T :: Iter ; fn to_socket_addrs (& self) -> io :: Result < T :: Iter > { (* * self) . to_socket_addrs () } }
};
}
