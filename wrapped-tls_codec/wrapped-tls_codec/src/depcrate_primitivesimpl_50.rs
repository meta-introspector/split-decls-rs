// Generated macro for impl_50 (impl)
macro_rules! Depcrate_primitivesimpl_50 {
() => {
// Module: crate::primitives
// Provides: {"impl_50"}
// Dependencies: {}
impl < T > DeserializeBytes for PhantomData < T > { # [inline (always)] fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { Ok ((PhantomData , bytes)) } }
};
}
