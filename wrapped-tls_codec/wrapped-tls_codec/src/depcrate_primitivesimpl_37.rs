// Generated macro for impl_37 (impl)
macro_rules! Depcrate_primitivesimpl_37 {
() => {
// Module: crate::primitives
// Provides: {"impl_37"}
// Dependencies: {}
impl < T , U > DeserializeBytes for (T , U) where T : DeserializeBytes , U : DeserializeBytes , { # [inline (always)] fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { let (first_element , remainder) = T :: tls_deserialize_bytes (bytes) ? ; let (second_element , remainder) = U :: tls_deserialize_bytes (remainder) ? ; Ok (((first_element , second_element) , remainder)) } }
};
}
