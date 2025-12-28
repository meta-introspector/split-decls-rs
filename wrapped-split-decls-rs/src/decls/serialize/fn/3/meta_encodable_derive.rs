use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub (super) fn meta_encodable_derive (mut s : synstructure :: Structure < '_ > ,) -> proc_macro2 :: TokenStream { if ! s . ast () . generics . lifetimes () . any (| lt | lt . lifetime . ident == "tcx") { s . add_impl_generic (parse_quote ! { 'tcx }) ; } s . add_impl_generic (parse_quote ! { '__a }) ; let encoder_ty = quote ! { EncodeContext <'__a , 'tcx > } ; s . add_bounds (synstructure :: AddBounds :: Generics) ; encodable_body (s , encoder_ty , true) }
}