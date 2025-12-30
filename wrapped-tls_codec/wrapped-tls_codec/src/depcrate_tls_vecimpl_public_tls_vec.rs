// Generated macro for impl_public_tls_vec (macro)
macro_rules! Depcrate_tls_vecimpl_public_tls_vec {
() => {
// Module: crate::tls_vec
// Provides: {"impl_public_tls_vec"}
// Dependencies: {}
macro_rules ! impl_public_tls_vec { ($ size : ty , $ name : ident , $ len_len : literal) => { impl_tls_vec_generic ! ($ size , $ name , $ len_len) ; impl_tls_vec_codec_generic ! ($ size , $ name , $ len_len) ; impl < T : Serialize > $ name < T > { impl_serialize_common ! (self , $ size , $ name , $ len_len , # [cfg (feature = "std")]) ; impl_serialize ! (self , $ size , $ name , $ len_len) ; } impl < T : Size > $ name < T > { impl_size ! (self , $ size , $ name , $ len_len) ; } impl < T : Deserialize > $ name < T > { impl_deserialize ! (self , $ size , $ name , $ len_len) ; } impl < T : DeserializeBytes > $ name < T > { impl_deserialize_bytes ! (self , $ size , $ name , $ len_len) ; } } ; }
};
}
