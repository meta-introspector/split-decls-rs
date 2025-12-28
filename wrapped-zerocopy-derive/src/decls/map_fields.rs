macro_rules! map_fields {
    () => {
        fn map_fields < 'a > (fields : impl 'a + IntoIterator < Item = & 'a Field > ,) -> Vec < (& 'a Visibility , TokenStream , & 'a Type) > { fields . into_iter () . enumerate () . map (| (idx , f) | { (& f . vis , f . ident . as_ref () . map (ToTokens :: to_token_stream) . unwrap_or_else (| | Index :: from (idx) . to_token_stream ()) , & f . ty ,) }) . collect () }
    };
}

map_fields!();