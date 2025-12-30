// Generated macro for impl_36 (impl)
macro_rules! Depcrate_primitivesimpl_36 {
() => {
// Module: crate::primitives
// Provides: {"impl_36"}
// Dependencies: {}
impl < T , U > Deserialize for (T , U) where T : Deserialize , U : Deserialize , { # [cfg (feature = "std")] # [inline (always)] fn tls_deserialize < R : Read > (bytes : & mut R) -> Result < Self , Error > { Ok ((T :: tls_deserialize (bytes) ? , U :: tls_deserialize (bytes) ?)) } }
};
}
