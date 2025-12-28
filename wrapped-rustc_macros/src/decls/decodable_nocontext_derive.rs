macro_rules! decodable_nocontext_derive {
    () => {
        pub (super) fn decodable_nocontext_derive (mut s : synstructure :: Structure < '_ > ,) -> proc_macro2 :: TokenStream { let decoder_ty = quote ! { __D } ; s . add_impl_generic (parse_quote ! { # decoder_ty : :: rustc_serialize :: Decoder }) ; s . add_bounds (synstructure :: AddBounds :: Fields) ; decodable_body (s , decoder_ty) }
    };
}

decodable_nocontext_derive!()