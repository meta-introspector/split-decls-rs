// Generated macro for impl_tls_vec_codec_bytes (macro)
macro_rules! Depcrate_tls_vecimpl_tls_vec_codec_bytes {
() => {
// Module: crate::tls_vec
// Provides: {"impl_tls_vec_codec_bytes"}
// Dependencies: {}
macro_rules ! impl_tls_vec_codec_bytes { ($ size : ty , $ name : ident , $ len_len : literal) => { impl Serialize for $ name { # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { self . serialize_bytes (writer) } } impl Size for $ name { # [inline] fn tls_serialized_len (& self) -> usize { self . tls_serialized_byte_length () } } impl Serialize for &$ name { # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { self . serialize_bytes (writer) } } impl Size for &$ name { # [inline] fn tls_serialized_len (& self) -> usize { self . tls_serialized_byte_length () } } impl Deserialize for $ name { # [cfg (feature = "std")] fn tls_deserialize < R : Read > (bytes : & mut R) -> Result < Self , Error > { Self :: deserialize_bytes (bytes) } } impl DeserializeBytes for $ name { fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { Self :: deserialize_bytes_bytes (bytes) } } impl SerializeBytes for $ name { fn tls_serialize (& self) -> Result < Vec < u8 >, Error > { self . serialize_bytes_bytes () } } } ; }
};
}
