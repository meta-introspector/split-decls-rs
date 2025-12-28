macro_rules! collect_lifetimes_from_tokens {
    () => {
        fn collect_lifetimes_from_tokens (tokens : TokenStream , out : & mut BTreeSet < syn :: Lifetime >) { let mut iter = tokens . into_iter () ; while let Some (tt) = iter . next () { match & tt { TokenTree :: Punct (op) if op . as_char () == '\'' && op . spacing () == Spacing :: Joint => { if let Some (TokenTree :: Ident (ident)) = iter . next () { out . insert (syn :: Lifetime { apostrophe : op . span () , ident , }) ; } } TokenTree :: Group (group) => { let tokens = group . stream () ; collect_lifetimes_from_tokens (tokens , out) ; } _ => { } } } }
    };
}

collect_lifetimes_from_tokens!();