// Generated macro for impl_deserialize (macro)
macro_rules! Depcrate_tls_vecimpl_deserialize {
() => {
// Module: crate::tls_vec
// Provides: {"impl_deserialize"}
// Dependencies: {}
macro_rules ! impl_deserialize { ($ self : ident , $ size : ty , $ name : ident , $ len_len : literal) => { # [cfg (feature = "std")] # [inline (always)] fn deserialize < R : Read > (bytes : & mut R) -> Result < Self , Error > { let mut result = Self { vec : Vec :: new () } ; let len = <$ size >:: tls_deserialize (bytes) ?; let mut read = len . tls_serialized_len () ; let len_len = read ; while (read - len_len) < len . try_into () . unwrap () { let element = T :: tls_deserialize (bytes) ?; read += element . tls_serialized_len () ; result . push (element) ; } Ok (result) } } ; }
};
}
