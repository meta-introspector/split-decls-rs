// Generated macro for impl_194 (impl)
macro_rules! Depcrate_net_addrimpl_194 {
() => {
// Module: crate::net::addr
// Provides: {"impl_194"}
// Dependencies: {}
impl sealed :: ToSocketAddrsPriv for SocketAddrV6 { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { SocketAddr :: V6 (* self) . to_socket_addrs (sealed :: Internal) } }
};
}
