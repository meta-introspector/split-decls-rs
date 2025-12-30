// Generated macro for impl_79 (impl)
macro_rules! Depcrate_quic_vecimpl_79 {
() => {
// Module: crate::quic_vec
// Provides: {"impl_79"}
// Dependencies: {}
impl < T : SerializeBytes > SerializeBytes for Vec < T > { fn tls_serialize (& self) -> Result < Vec < u8 > , Error > { self . as_slice () . tls_serialize () } }
};
}
