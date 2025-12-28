macro_rules! try_join_tokens {
    () => {
        fn try_join_tokens (exprs : impl ExactSizeIterator < Item = syn :: Result < impl ToTokens > > , sep : impl ToTokens ,) -> syn :: Result < TokenStream > { let expr_count = exprs . len () ; let mut out_tokens = TokenStream :: new () ; for (i , expr) in exprs . enumerate () { expr ? . to_tokens (& mut out_tokens) ; if expr_count != i + 1 { sep . to_tokens (& mut out_tokens) ; } } Ok (out_tokens) }
    };
}

try_join_tokens!()