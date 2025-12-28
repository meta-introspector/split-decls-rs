macro_rules! respan_token {
    () => {
        fn respan_token (mut token : proc_macro2 :: TokenTree , span : proc_macro2 :: Span ,) -> proc_macro2 :: TokenTree { if let proc_macro2 :: TokenTree :: Group (g) = & mut token { * g = proc_macro2 :: Group :: new (g . delimiter () , respan_tokenstream (g . stream () , span)) ; } token . set_span (span) ; token }
    };
}

respan_token!()