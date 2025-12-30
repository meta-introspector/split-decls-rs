// Generated macro for encodable_nocontext_derive (function)
macro_rules! Depcrate_serializeencodable_nocontext_derive {
() => {
// Module: crate::serialize
// Provides: {"encodable_nocontext_derive"}
// Dependencies: {}
pub (super) fn encodable_nocontext_derive (mut s : synstructure :: Structure < '_ > ,) -> proc_macro2 :: TokenStream { let encoder_ty = quote ! { __E } ; s . add_impl_generic (parse_quote ! { # encoder_ty : :: rustc_serialize :: Encoder }) ; s . add_bounds (synstructure :: AddBounds :: Fields) ; encodable_body (s , encoder_ty , false) }
};
}
