// Generated macro for read_variable_length_bytes (function)
macro_rules! Depcrate_quic_vecread_variable_length_bytes {
() => {
// Module: crate::quic_vec
// Provides: {"read_variable_length_bytes"}
// Dependencies: {}
# [inline (always)] fn read_variable_length_bytes (bytes : & [u8]) -> Result < ((usize , usize) , & [u8]) , Error > { let (len_len_byte , mut remainder) = u8 :: tls_deserialize_bytes (bytes) ? ; let (mut length , len_len) = calculate_length (len_len_byte) ? ; for _ in 1 .. len_len { let (next , next_remainder) = u8 :: tls_deserialize_bytes (remainder) ? ; remainder = next_remainder ; length = (length << 8) + usize :: from (next) ; } check_min_length (length , len_len) ? ; Ok (((length , len_len) , remainder)) }
};
}
