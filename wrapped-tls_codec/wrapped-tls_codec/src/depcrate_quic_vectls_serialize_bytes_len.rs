// Generated macro for tls_serialize_bytes_len (function)
macro_rules! Depcrate_quic_vectls_serialize_bytes_len {
() => {
// Module: crate::quic_vec
// Provides: {"tls_serialize_bytes_len"}
// Dependencies: {}
# [inline (always)] fn tls_serialize_bytes_len (bytes : & [u8]) -> usize { let content_length = bytes . len () ; let len_len = length_encoding_bytes (content_length as u64) . unwrap_or ({ 0 }) ; content_length + len_len }
};
}
