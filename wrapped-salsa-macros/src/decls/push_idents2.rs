macro_rules! push_idents2 {
    () => {
        fn push_idents2 (input : proc_macro2 :: TokenStream , user_tokens : & mut HashSet < String >) { input . into_iter () . for_each (| token | match token { proc_macro2 :: TokenTree :: Group (g) => { push_idents2 (g . stream () , user_tokens) ; } proc_macro2 :: TokenTree :: Ident (ident) => { user_tokens . insert (ident . to_string ()) ; } proc_macro2 :: TokenTree :: Punct (_) => () , proc_macro2 :: TokenTree :: Literal (_) => () , }) }
    };
}

push_idents2!();