// Generated macro for impl_tls_vec_codec_generic (macro)
macro_rules! Depcrate_tls_vecimpl_tls_vec_codec_generic {
() => {
// Module: crate::tls_vec
// Provides: {"impl_tls_vec_codec_generic"}
// Dependencies: {}
macro_rules ! impl_tls_vec_codec_generic { ($ size : ty , $ name : ident , $ len_len : literal $ (, $ bounds : ident) *) => { impl < T : $ ($ bounds +) * Serialize > Serialize for $ name < T > { # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { self . serialize (writer) } } impl < T : $ ($ bounds +) * Size > Size for $ name < T > { # [inline] fn tls_serialized_len (& self) -> usize { self . tls_serialized_length () } } impl < T : $ ($ bounds +) * Serialize > Serialize for &$ name < T > { # [cfg (feature = "std")] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { self . serialize (writer) } } impl < T : $ ($ bounds +) * Size > Size for &$ name < T > { # [inline] fn tls_serialized_len (& self) -> usize { self . tls_serialized_length () } } impl < T : $ ($ bounds +) * Deserialize > Deserialize for $ name < T > { # [cfg (feature = "std")] fn tls_deserialize < R : Read > (bytes : & mut R) -> Result < Self , Error > { Self :: deserialize (bytes) } } impl < T : $ ($ bounds +) * DeserializeBytes > DeserializeBytes for $ name < T > { fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { Self :: deserialize_bytes (bytes) } } } ; }
};
}
