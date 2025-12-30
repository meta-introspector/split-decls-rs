// Generated macro for impl_77 (impl)
macro_rules! Depcrate_quic_vecimpl_77 {
() => {
// Module: crate::quic_vec
// Provides: {"impl_77"}
// Dependencies: {}
impl < T : SerializeBytes > SerializeBytes for & [T] { # [inline (always)] fn tls_serialize (& self) -> Result < Vec < u8 > , Error > { let content_length = self . iter () . fold (0 , | acc , e | acc + e . tls_serialized_len ()) ; let mut length = write_variable_length (content_length) ? ; let len_len = length . len () ; let mut out = Vec :: with_capacity (content_length + len_len) ; out . append (& mut length) ; for e in self . iter () { out . append (& mut e . tls_serialize () ?) ; } # [cfg (debug_assertions)] if out . len () - len_len != content_length { return Err (Error :: LibraryError) ; } Ok (out) } }
};
}
