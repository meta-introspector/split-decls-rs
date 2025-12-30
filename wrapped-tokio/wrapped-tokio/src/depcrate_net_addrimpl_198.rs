// Generated macro for impl_198 (impl)
macro_rules! Depcrate_net_addrimpl_198 {
() => {
// Module: crate::net::addr
// Provides: {"impl_198"}
// Dependencies: {}
impl sealed :: ToSocketAddrsPriv for (Ipv4Addr , u16) { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { let (ip , port) = * self ; SocketAddrV4 :: new (ip , port) . to_socket_addrs (sealed :: Internal) } }
};
}
