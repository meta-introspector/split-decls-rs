// Generated macro for impl_188 (impl)
macro_rules! Depcrate_net_addrimpl_188 {
() => {
// Module: crate::net::addr
// Provides: {"impl_188"}
// Dependencies: {}
impl < T > sealed :: ToSocketAddrsPriv for & T where T : sealed :: ToSocketAddrsPriv + ? Sized , { type Iter = T :: Iter ; type Future = T :: Future ; fn to_socket_addrs (& self , _ : sealed :: Internal) -> Self :: Future { (* * self) . to_socket_addrs (sealed :: Internal) } }
};
}
