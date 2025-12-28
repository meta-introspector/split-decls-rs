macro_rules! join_tokens {
    () => {
        fn join_tokens (exprs : impl ExactSizeIterator < Item = impl ToTokens > , sep : impl ToTokens ,) -> TokenStream { let expr_count = exprs . len () ; let mut out_tokens = TokenStream :: new () ; for (i , expr) in exprs . enumerate () { expr . to_tokens (& mut out_tokens) ; if expr_count != i + 1 { sep . to_tokens (& mut out_tokens) ; } } out_tokens }
    };
}

join_tokens!();