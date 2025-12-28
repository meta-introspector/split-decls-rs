use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub (super) fn type_encodable_derive (mut s : synstructure :: Structure < '_ > ,) -> proc_macro2 :: TokenStream { let encoder_ty = quote ! { __E } ; if ! s . ast () . generics . lifetimes () . any (| lt | lt . lifetime . ident == "tcx") { s . add_impl_generic (parse_quote ! { 'tcx }) ; } s . add_impl_generic (parse_quote ! { # encoder_ty : :: rustc_middle :: ty :: codec :: TyEncoder <'tcx > }) ; s . add_bounds (synstructure :: AddBounds :: Fields) ; encodable_body (s , encoder_ty , false) }
}