// Generated macro for impl_41 (impl)
macro_rules! Depcrate_primitivesimpl_41 {
() => {
// Module: crate::primitives
// Provides: {"impl_41"}
// Dependencies: {}
impl < T , U , V > DeserializeBytes for (T , U , V) where T : DeserializeBytes , U : DeserializeBytes , V : DeserializeBytes , { # [inline (always)] fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { let (first_element , remainder) = T :: tls_deserialize_bytes (bytes) ? ; let (second_element , remainder) = U :: tls_deserialize_bytes (remainder) ? ; let (third_element , remainder) = V :: tls_deserialize_bytes (remainder) ? ; Ok (((first_element , second_element , third_element) , remainder)) } }
};
}
