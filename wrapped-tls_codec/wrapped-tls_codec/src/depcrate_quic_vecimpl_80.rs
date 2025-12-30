// Generated macro for impl_80 (impl)
macro_rules! Depcrate_quic_vecimpl_80 {
() => {
// Module: crate::quic_vec
// Provides: {"impl_80"}
// Dependencies: {}
impl < T : Size > Size for & [T] { # [inline (always)] fn tls_serialized_len (& self) -> usize { let content_length = self . iter () . fold (0 , | acc , e | acc + e . tls_serialized_len ()) ; let len_len = length_encoding_bytes (content_length as u64) . unwrap_or ({ 0 }) ; content_length + len_len } }
};
}
