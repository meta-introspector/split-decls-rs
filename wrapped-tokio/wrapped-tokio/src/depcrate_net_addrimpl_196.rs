// Generated macro for impl_196 (impl)
macro_rules! Depcrate_net_addrimpl_196 {
() => {
// Module: crate::net::addr
// Provides: {"impl_196"}
// Dependencies: {}
impl sealed :: ToSocketAddrsPriv for (IpAddr , u16) { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { let iter = Some (SocketAddr :: from (* self)) . into_iter () ; future :: ready (Ok (iter)) } }
};
}
