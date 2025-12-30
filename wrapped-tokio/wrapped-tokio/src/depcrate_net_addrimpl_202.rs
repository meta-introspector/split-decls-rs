// Generated macro for impl_202 (impl)
macro_rules! Depcrate_net_addrimpl_202 {
() => {
// Module: crate::net::addr
// Provides: {"impl_202"}
// Dependencies: {}
impl sealed :: ToSocketAddrsPriv for & [SocketAddr] { type Iter = std :: vec :: IntoIter < SocketAddr > ; type Future = ReadyFuture < Self :: Iter > ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { # [inline] fn slice_to_vec (addrs : & [SocketAddr]) -> Vec < SocketAddr > { addrs . to_vec () } let iter = slice_to_vec (self) . into_iter () ; future :: ready (Ok (iter)) } }
};
}
