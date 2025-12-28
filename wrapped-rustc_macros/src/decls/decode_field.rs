macro_rules! decode_field {
    () => {
        fn decode_field (field : & syn :: Field) -> proc_macro2 :: TokenStream { let field_span = field . ident . as_ref () . map_or (field . ty . span () , | ident | ident . span ()) ; let decode_inner_method = if let syn :: Type :: Reference (_) = field . ty { quote ! { :: rustc_middle :: ty :: codec :: RefDecodable :: decode } } else { quote ! { :: rustc_serialize :: Decodable :: decode } } ; let __decoder = quote ! { __decoder } ; quote_spanned ! { field_span => # decode_inner_method (# __decoder) } }
    };
}

decode_field!()