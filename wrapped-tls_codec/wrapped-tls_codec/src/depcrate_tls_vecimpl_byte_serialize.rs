// Generated macro for impl_byte_serialize (macro)
macro_rules! Depcrate_tls_vecimpl_byte_serialize {
() => {
// Module: crate::tls_vec
// Provides: {"impl_byte_serialize"}
// Dependencies: {}
macro_rules ! impl_byte_serialize { ($ self : ident , $ size : ty , $ name : ident , $ len_len : literal) => { # [cfg (feature = "std")] # [inline (always)] fn serialize_bytes < W : Write > (&$ self , writer : & mut W) -> Result < usize , Error > { let (tls_serialized_len , byte_length) = $ self . get_content_lengths () ?; let mut written = <$ size as Serialize >:: tls_serialize (&<$ size >:: try_from (byte_length) . unwrap () , writer) ?; written += writer . write ($ self . as_slice ()) ?; $ self . assert_written_bytes (tls_serialized_len , written) ?; Ok (written) } } ; }
};
}
