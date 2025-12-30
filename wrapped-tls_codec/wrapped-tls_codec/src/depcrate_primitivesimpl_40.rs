// Generated macro for impl_40 (impl)
macro_rules! Depcrate_primitivesimpl_40 {
() => {
// Module: crate::primitives
// Provides: {"impl_40"}
// Dependencies: {}
impl < T , U , V > Deserialize for (T , U , V) where T : Deserialize , U : Deserialize , V : Deserialize , { # [cfg (feature = "std")] # [inline (always)] fn tls_deserialize < R : Read > (bytes : & mut R) -> Result < Self , Error > { Ok ((T :: tls_deserialize (bytes) ? , U :: tls_deserialize (bytes) ? , V :: tls_deserialize (bytes) ? ,)) } }
};
}
