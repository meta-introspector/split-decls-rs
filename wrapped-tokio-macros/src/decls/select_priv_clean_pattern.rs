macro_rules! select_priv_clean_pattern {
    () => {
        # [doc = " Implementation detail of the `select!` macro. This macro is **not** intended"] # [doc = " to be used as part of the public API and is permitted to change."] # [proc_macro] # [doc (hidden)] pub fn select_priv_clean_pattern (input : TokenStream) -> TokenStream { select :: clean_pattern_macro (input) }
    };
}

select_priv_clean_pattern!()