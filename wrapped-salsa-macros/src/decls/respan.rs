macro_rules! respan {
    () => {
        fn respan < T > (t : & T , span : proc_macro2 :: Span) -> T where T : ToTokens + Spanned + syn :: parse :: Parse , { let tokens = t . to_token_stream () ; let respanned = respan_tokenstream (tokens , span) ; syn :: parse2 (respanned) . unwrap () }
    };
}

respan!()