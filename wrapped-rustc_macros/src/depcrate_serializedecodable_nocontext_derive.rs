// Generated macro for decodable_nocontext_derive (function)
macro_rules! Depcrate_serializedecodable_nocontext_derive {
() => {
// Module: crate::serialize
// Provides: {"decodable_nocontext_derive"}
// Dependencies: {}
pub (super) fn decodable_nocontext_derive (mut s : synstructure :: Structure < '_ > ,) -> proc_macro2 :: TokenStream { let decoder_ty = quote ! { __D } ; s . add_impl_generic (parse_quote ! { # decoder_ty : :: rustc_serialize :: Decoder }) ; s . add_bounds (synstructure :: AddBounds :: Fields) ; decodable_body (s , decoder_ty) }
};
}
