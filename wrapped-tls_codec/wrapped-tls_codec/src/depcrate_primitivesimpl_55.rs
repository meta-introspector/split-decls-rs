// Generated macro for impl_55 (impl)
macro_rules! Depcrate_primitivesimpl_55 {
() => {
// Module: crate::primitives
// Provides: {"impl_55"}
// Dependencies: {}
impl < T : SerializeBytes > SerializeBytes for Box < T > { # [inline (always)] fn tls_serialize (& self) -> Result < Vec < u8 > , Error > { self . as_ref () . tls_serialize () } }
};
}
