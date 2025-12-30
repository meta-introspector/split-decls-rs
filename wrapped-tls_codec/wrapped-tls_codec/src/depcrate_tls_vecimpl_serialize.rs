// Generated macro for impl_serialize (macro)
macro_rules! Depcrate_tls_vecimpl_serialize {
() => {
// Module: crate::tls_vec
// Provides: {"impl_serialize"}
// Dependencies: {}
macro_rules ! impl_serialize { ($ self : ident , $ size : ty , $ name : ident , $ len_len : literal) => { # [cfg (feature = "std")] # [inline (always)] fn serialize < W : Write > (&$ self , writer : & mut W) -> Result < usize , Error > { let (tls_serialized_len , byte_length) = $ self . get_content_lengths () ?; let mut written = <$ size as Serialize >:: tls_serialize (&<$ size >:: try_from (byte_length) . unwrap () , writer) ?; for e in $ self . as_slice () . iter () { written += e . tls_serialize (writer) ?; } $ self . assert_written_bytes (tls_serialized_len , written) ?; Ok (written) } } ; }
};
}
