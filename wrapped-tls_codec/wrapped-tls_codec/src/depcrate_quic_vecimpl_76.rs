// Generated macro for impl_76 (impl)
macro_rules! Depcrate_quic_vecimpl_76 {
() => {
// Module: crate::quic_vec
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : DeserializeBytes > DeserializeBytes for Vec < T > { # [inline (always)] fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { let ((length , len_len) , mut remainder) = read_variable_length_bytes (bytes) ? ; if length == 0 { return Ok ((Vec :: new () , remainder)) ; } let mut result = Vec :: new () ; let mut read = len_len ; while (read - len_len) < length { let (element , next_remainder) = T :: tls_deserialize_bytes (remainder) ? ; remainder = next_remainder ; read += element . tls_serialized_len () ; result . push (element) ; } Ok ((result , remainder)) } }
};
}
