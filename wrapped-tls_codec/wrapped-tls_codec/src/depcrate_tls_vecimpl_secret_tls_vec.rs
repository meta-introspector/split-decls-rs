// Generated macro for impl_secret_tls_vec (macro)
macro_rules! Depcrate_tls_vecimpl_secret_tls_vec {
() => {
// Module: crate::tls_vec
// Provides: {"impl_secret_tls_vec"}
// Dependencies: {}
macro_rules ! impl_secret_tls_vec { ($ size : ty , $ name : ident , $ len_len : literal) => { impl_tls_vec_generic ! ($ size , $ name , $ len_len , Zeroize) ; impl_tls_vec_codec_generic ! ($ size , $ name , $ len_len , Zeroize) ; impl < T : Serialize + Zeroize > $ name < T > { impl_serialize_common ! (self , $ size , $ name , $ len_len , # [cfg (feature = "std")]) ; impl_serialize ! (self , $ size , $ name , $ len_len) ; } impl < T : Size + Zeroize > $ name < T > { impl_size ! (self , $ size , $ name , $ len_len) ; } impl < T : Deserialize + Zeroize > $ name < T > { impl_deserialize ! (self , $ size , $ name , $ len_len) ; } impl < T : DeserializeBytes + Zeroize > $ name < T > { impl_deserialize_bytes ! (self , $ size , $ name , $ len_len) ; } impl < T : Zeroize > Zeroize for $ name < T > { fn zeroize (& mut self) { self . vec . zeroize () } } impl < T : Zeroize > Drop for $ name < T > { fn drop (& mut self) { self . zeroize () } } } ; }
};
}
