macro_rules! clean_pattern_macro {
    () => {
        pub (crate) fn clean_pattern_macro (input : TokenStream) -> TokenStream { let mut input : syn :: Pat = match syn :: Pat :: parse_single . parse (input . clone ()) { Ok (it) => it , Err (_) => return input , } ; clean_pattern (& mut input) ; quote :: ToTokens :: into_token_stream (input) . into () }
    };
}

clean_pattern_macro!()