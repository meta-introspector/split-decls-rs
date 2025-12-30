// Generated macro for impl_56 (impl)
macro_rules! Depcrate_primitivesimpl_56 {
() => {
// Module: crate::primitives
// Provides: {"impl_56"}
// Dependencies: {}
impl < T : Deserialize > Deserialize for Box < T > { # [cfg (feature = "std")] # [inline (always)] fn tls_deserialize < R : Read > (bytes : & mut R) -> Result < Self , Error > { T :: tls_deserialize (bytes) . map (Box :: new) } }
};
}
