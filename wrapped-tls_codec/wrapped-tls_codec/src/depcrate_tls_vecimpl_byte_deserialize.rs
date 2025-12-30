// Generated macro for impl_byte_deserialize (macro)
macro_rules! Depcrate_tls_vecimpl_byte_deserialize {
() => {
// Module: crate::tls_vec
// Provides: {"impl_byte_deserialize"}
// Dependencies: {}
macro_rules ! impl_byte_deserialize { ($ self : ident , $ size : ty , $ name : ident , $ len_len : literal) => { # [cfg (feature = "std")] # [inline (always)] fn deserialize_bytes < R : Read > (bytes : & mut R) -> Result < Self , Error > { let len = <$ size >:: tls_deserialize (bytes) ?. try_into () . unwrap () ; if cfg ! (fuzzing) && len > u16 :: MAX as usize { return Err (Error :: DecodingError (format ! ("Trying to allocate {} bytes. Only {} allowed." , len , u16 :: MAX))) ; } let mut result = Self { vec : vec ! [0u8 ; len] , } ; bytes . read_exact (result . vec . as_mut_slice ()) ?; Ok (result) } # [inline (always)] fn deserialize_bytes_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { let (type_len , remainder) = <$ size >:: tls_deserialize_bytes (bytes) ?; let len = type_len . try_into () . unwrap () ; if cfg ! (fuzzing) && len > u16 :: MAX as usize { return Err (Error :: DecodingError (alloc :: format ! ("Trying to allocate {} bytes. Only {} allowed." , len , u16 :: MAX))) ; } let vec = bytes . get ($ len_len .. len + $ len_len) . ok_or (Error :: EndOfStream) ?; let result = Self { vec : vec . to_vec () } ; Ok ((result , & remainder . get (len ..) . ok_or (Error :: EndOfStream) ?)) } } ; }
};
}
