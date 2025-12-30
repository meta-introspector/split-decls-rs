// Generated macro for impl_190 (impl)
macro_rules! Depcrate_net_addrimpl_190 {
() => {
// Module: crate::net::addr
// Provides: {"impl_190"}
// Dependencies: {}
impl sealed :: ToSocketAddrsPriv for SocketAddr { type Iter = std :: option :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { let iter = Some (* self) . into_iter () ; future :: ready (Ok (iter)) } }
};
}
