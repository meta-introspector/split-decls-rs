// Generated macro for impl_57 (impl)
macro_rules! Depcrate_primitivesimpl_57 {
() => {
// Module: crate::primitives
// Provides: {"impl_57"}
// Dependencies: {}
impl < T : DeserializeBytes > DeserializeBytes for Box < T > { # [inline (always)] fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { T :: tls_deserialize_bytes (bytes) . map (| (v , r) | (Box :: new (v) , r)) } }
};
}
