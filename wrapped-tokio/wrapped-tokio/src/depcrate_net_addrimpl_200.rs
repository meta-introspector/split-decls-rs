// Generated macro for impl_200 (impl)
macro_rules! Depcrate_net_addrimpl_200 {
() => {
// Module: crate::net::addr
// Provides: {"impl_200"}
// Dependencies: {}
impl sealed :: ToSocketAddrsPriv for (Ipv6Addr , u16) { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { let (ip , port) = * self ; SocketAddrV6 :: new (ip , port , 0 , 0) . to_socket_addrs (sealed :: Internal) } }
};
}
