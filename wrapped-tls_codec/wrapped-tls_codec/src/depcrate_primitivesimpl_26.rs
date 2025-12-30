// Generated macro for impl_26 (impl)
macro_rules! Depcrate_primitivesimpl_26 {
() => {
// Module: crate::primitives
// Provides: {"impl_26"}
// Dependencies: {}
impl < T : SerializeBytes > SerializeBytes for & Option < T > { # [inline] fn tls_serialize (& self) -> Result < Vec < u8 > , Error > { (* self) . tls_serialize () } }
};
}
