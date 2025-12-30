// Generated macro for impl_1652 (impl)
macro_rules! Depcrate_hash_hsimpl_1652 {
() => {
// Module: crate::hash_hs
// Provides: {"impl_1652"}
// Dependencies: {}
impl Clone for HandshakeHash { fn clone (& self) -> Self { Self { provider : self . provider , ctx : self . ctx . fork () , client_auth : self . client_auth . clone () , } } }
};
}
