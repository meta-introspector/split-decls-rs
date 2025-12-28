macro_rules! type_decodable_derive {
    () => {
        pub (super) fn type_decodable_derive (mut s : synstructure :: Structure < '_ > ,) -> proc_macro2 :: TokenStream { if ! s . ast () . generics . lifetimes () . any (| lt | lt . lifetime . ident == "tcx") { s . add_impl_generic (parse_quote ! { 'tcx }) ; } let decoder_ty = quote ! { __D } ; s . add_impl_generic (parse_quote ! { # decoder_ty : :: rustc_middle :: ty :: codec :: TyDecoder <'tcx > }) ; s . add_bounds (synstructure :: AddBounds :: Fields) ; decodable_body (s , decoder_ty) }
    };
}

type_decodable_derive!()