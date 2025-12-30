// Generated macro for impl_size (macro)
macro_rules! Depcrate_tls_vecimpl_size {
() => {
// Module: crate::tls_vec
// Provides: {"impl_size"}
// Dependencies: {}
macro_rules ! impl_size { ($ self : ident , $ size : ty , $ name : ident , $ len_len : literal) => { # [doc = " The serialized len"] # [inline (always)] fn tls_serialized_length (&$ self) -> usize { $ self . as_slice () . iter () . fold ($ len_len , | acc , e | acc + e . tls_serialized_len ()) } } }
};
}
