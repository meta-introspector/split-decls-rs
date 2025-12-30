// Generated macro for impl_49 (impl)
macro_rules! Depcrate_primitivesimpl_49 {
() => {
// Module: crate::primitives
// Provides: {"impl_49"}
// Dependencies: {}
impl < T > Deserialize for PhantomData < T > { # [cfg (feature = "std")] # [inline (always)] fn tls_deserialize < R : Read > (_ : & mut R) -> Result < Self , Error > { Ok (PhantomData) } }
};
}
