// Generated macro for impl_deserialize_bytes (macro)
macro_rules! Depcrate_tls_vecimpl_deserialize_bytes {
() => {
// Module: crate::tls_vec
// Provides: {"impl_deserialize_bytes"}
// Dependencies: {}
macro_rules ! impl_deserialize_bytes { ($ self : ident , $ size : ty , $ name : ident , $ len_len : literal) => { # [inline (always)] fn deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { let mut result = Self { vec : Vec :: new () } ; let (len , mut remainder) = <$ size >:: tls_deserialize_bytes (bytes) ?; let mut read = len . tls_serialized_len () ; let len_len = read ; while (read - len_len) < len . try_into () . unwrap () { let (element , next_remainder) = T :: tls_deserialize_bytes (remainder) ?; remainder = next_remainder ; read += element . tls_serialized_len () ; result . push (element) ; } Ok ((result , remainder)) } } ; }
};
}
