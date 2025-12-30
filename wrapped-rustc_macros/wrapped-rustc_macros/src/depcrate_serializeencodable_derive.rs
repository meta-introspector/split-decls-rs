// Generated macro for encodable_derive (function)
macro_rules! Depcrate_serializeencodable_derive {
() => {
// Module: crate::serialize
// Provides: {"encodable_derive"}
// Dependencies: {}
pub (super) fn encodable_derive (mut s : synstructure :: Structure < '_ >) -> proc_macro2 :: TokenStream { let encoder_ty = quote ! { __E } ; s . add_impl_generic (parse_quote ! { # encoder_ty : :: rustc_span :: SpanEncoder }) ; s . add_bounds (synstructure :: AddBounds :: Generics) ; encodable_body (s , encoder_ty , false) }
};
}
