// Generated macro for impl_serialize_bytes_bytes (macro)
macro_rules! Depcrate_tls_vecimpl_serialize_bytes_bytes {
() => {
// Module: crate::tls_vec
// Provides: {"impl_serialize_bytes_bytes"}
// Dependencies: {}
macro_rules ! impl_serialize_bytes_bytes { ($ self : ident , $ size : ty , $ name : ident , $ len_len : literal) => { fn serialize_bytes_bytes (&$ self) -> Result < Vec < u8 >, Error > { let (tls_serialized_len , byte_length) = $ self . get_content_lengths () ?; let mut vec = Vec ::< u8 >:: with_capacity (tls_serialized_len) ; let length_vec = <$ size as SerializeBytes >:: tls_serialize (& byte_length . try_into () . unwrap ()) ?; let mut written = length_vec . len () ; vec . extend_from_slice (& length_vec) ; let bytes = $ self . as_slice () ; vec . extend_from_slice (bytes) ; written += bytes . len () ; $ self . assert_written_bytes (tls_serialized_len , written) ?; Ok (vec) } } ; }
};
}
