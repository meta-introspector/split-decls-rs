macro_rules! allow_incompatible_msrv {
    () => {
        fn allow_incompatible_msrv (input : TokenStream) -> TokenStream { TokenStream :: from_iter (vec ! [TokenTree :: Punct (Punct :: new ('#' , Spacing :: Alone)) , TokenTree :: Group (Group :: new (Delimiter :: Bracket , TokenStream :: from_iter (vec ! [TokenTree :: Ident (Ident :: new ("allow" , Span :: call_site ())) , TokenTree :: Group (Group :: new (Delimiter :: Parenthesis , TokenStream :: from_iter (vec ! [TokenTree :: Ident (Ident :: new ("clippy" , Span :: call_site ())) , TokenTree :: Punct (Punct :: new (':' , Spacing :: Joint)) , TokenTree :: Punct (Punct :: new (':' , Spacing :: Alone)) , TokenTree :: Ident (Ident :: new ("incompatible_msrv" , Span :: call_site ())) ,]) ,)) ,]) ,)) ,] . into_iter () . chain (input) ,) }
    };
}

allow_incompatible_msrv!()