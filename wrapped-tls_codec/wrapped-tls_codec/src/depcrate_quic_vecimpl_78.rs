// Generated macro for impl_78 (impl)
macro_rules! Depcrate_quic_vecimpl_78 {
() => {
// Module: crate::quic_vec
// Provides: {"impl_78"}
// Dependencies: {}
impl < T : SerializeBytes > SerializeBytes for & Vec < T > { # [inline (always)] fn tls_serialize (& self) -> Result < Vec < u8 > , Error > { self . as_slice () . tls_serialize () } }
};
}
