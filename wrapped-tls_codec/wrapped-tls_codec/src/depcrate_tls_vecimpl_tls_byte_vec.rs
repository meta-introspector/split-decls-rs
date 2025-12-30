// Generated macro for impl_tls_byte_vec (macro)
macro_rules! Depcrate_tls_vecimpl_tls_byte_vec {
() => {
// Module: crate::tls_vec
// Provides: {"impl_tls_byte_vec"}
// Dependencies: {}
macro_rules ! impl_tls_byte_vec { ($ size : ty , $ name : ident , $ len_len : literal) => { impl_tls_vec ! ($ name , $ len_len) ; impl $ name { impl_serialize_common ! (self , $ size , $ name , $ len_len) ; impl_byte_serialize ! (self , $ size , $ name , $ len_len) ; impl_serialize_bytes_bytes ! (self , $ size , $ name , $ len_len) ; impl_byte_size ! (self , $ size , $ name , $ len_len) ; impl_byte_deserialize ! (self , $ size , $ name , $ len_len) ; } impl_tls_vec_codec_bytes ! ($ size , $ name , $ len_len) ; } ; }
};
}
