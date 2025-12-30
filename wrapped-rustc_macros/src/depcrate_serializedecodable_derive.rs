// Generated macro for decodable_derive (function)
macro_rules! Depcrate_serializedecodable_derive {
() => {
// Module: crate::serialize
// Provides: {"decodable_derive"}
// Dependencies: {}
pub (super) fn decodable_derive (mut s : synstructure :: Structure < '_ >) -> proc_macro2 :: TokenStream { let decoder_ty = quote ! { __D } ; s . add_impl_generic (parse_quote ! { # decoder_ty : :: rustc_span :: SpanDecoder }) ; s . add_bounds (synstructure :: AddBounds :: Generics) ; decodable_body (s , decoder_ty) }
};
}
