// Generated macro for meta_decodable_derive (function)
macro_rules! Depcrate_serializemeta_decodable_derive {
() => {
// Module: crate::serialize
// Provides: {"meta_decodable_derive"}
// Dependencies: {}
pub (super) fn meta_decodable_derive (mut s : synstructure :: Structure < '_ > ,) -> proc_macro2 :: TokenStream { if ! s . ast () . generics . lifetimes () . any (| lt | lt . lifetime . ident == "tcx") { s . add_impl_generic (parse_quote ! { 'tcx }) ; } s . add_impl_generic (parse_quote ! { '__a }) ; let decoder_ty = quote ! { DecodeContext <'__a , 'tcx > } ; s . add_bounds (synstructure :: AddBounds :: Generics) ; decodable_body (s , decoder_ty) }
};
}
