// Generated macro for impl_192 (impl)
macro_rules! Depcrate_net_addrimpl_192 {
() => {
// Module: crate::net::addr
// Provides: {"impl_192"}
// Dependencies: {}
impl sealed :: ToSocketAddrsPriv for SocketAddrV4 { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { SocketAddr :: V4 (* self) . to_socket_addrs (sealed :: Internal) } }
};
}
