macro_rules! decodable_derive {
    () => {
        pub (super) fn decodable_derive (mut s : synstructure :: Structure < '_ >) -> proc_macro2 :: TokenStream { let decoder_ty = quote ! { __D } ; s . add_impl_generic (parse_quote ! { # decoder_ty : :: rustc_span :: SpanDecoder }) ; s . add_bounds (synstructure :: AddBounds :: Generics) ; decodable_body (s , decoder_ty) }
    };
}

decodable_derive!()