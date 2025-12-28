macro_rules! respan_tokenstream {
    () => {
        fn respan_tokenstream (stream : proc_macro2 :: TokenStream , span : proc_macro2 :: Span ,) -> proc_macro2 :: TokenStream { stream . into_iter () . map (| token | respan_token (token , span)) . collect () }
    };
}

respan_tokenstream!();