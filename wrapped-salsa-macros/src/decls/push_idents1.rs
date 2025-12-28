macro_rules! push_idents1 {
    () => {
        fn push_idents1 (input : proc_macro :: TokenStream , user_tokens : & mut HashSet < String >) { input . into_iter () . for_each (| token | match token { proc_macro :: TokenTree :: Group (g) => { push_idents1 (g . stream () , user_tokens) ; } proc_macro :: TokenTree :: Ident (ident) => { user_tokens . insert (ident . to_string ()) ; } proc_macro :: TokenTree :: Punct (_) => () , proc_macro :: TokenTree :: Literal (_) => () , }) }
    };
}

push_idents1!();