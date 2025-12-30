// Generated macro for impl_serialize_common (macro)
macro_rules! Depcrate_tls_vecimpl_serialize_common {
() => {
// Module: crate::tls_vec
// Provides: {"impl_serialize_common"}
// Dependencies: {}
macro_rules ! impl_serialize_common { ($ self : ident , $ size : ty , $ name : ident , $ len_len : literal $ (,# [$ std_enabled : meta]) ?) => { $ (# [$ std_enabled]) ? fn get_content_lengths (&$ self) -> Result < (usize , usize) , Error > { let tls_serialized_len = $ self . tls_serialized_len () ; let byte_length = tls_serialized_len - $ len_len ; let max_len = <$ size >:: MAX . try_into () . unwrap () ; debug_assert ! (byte_length <= max_len , "Vector length can't be encoded in the vector length a {} >= {}" , byte_length , max_len) ; if byte_length > max_len { return Err (Error :: InvalidVectorLength) ; } Ok ((tls_serialized_len , byte_length)) } $ (# [$ std_enabled]) ? fn assert_written_bytes (&$ self , tls_serialized_len : usize , written : usize) -> Result < () , Error > { debug_assert_eq ! (written , tls_serialized_len , "{} bytes should have been serialized but {} were written" , tls_serialized_len , written) ; if written != tls_serialized_len { return Err (Error :: EncodingError (format ! ("{} bytes should have been serialized but {} were written" , tls_serialized_len , written))) ; } Ok (()) } } ; }
};
}
