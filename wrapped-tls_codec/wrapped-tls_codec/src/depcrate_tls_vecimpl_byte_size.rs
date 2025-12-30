// Generated macro for impl_byte_size (macro)
macro_rules! Depcrate_tls_vecimpl_byte_size {
() => {
// Module: crate::tls_vec
// Provides: {"impl_byte_size"}
// Dependencies: {}
macro_rules ! impl_byte_size { ($ self : ident , $ size : ty , $ name : ident , $ len_len : literal) => { # [doc = " The serialized len"] # [inline (always)] fn tls_serialized_byte_length (&$ self) -> usize { $ self . as_slice () . len () + $ len_len } } }
};
}
