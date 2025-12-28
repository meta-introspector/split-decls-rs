macro_rules! rustc_queries {
    () => {
        # [proc_macro] pub fn rustc_queries (input : TokenStream) -> TokenStream { query :: rustc_queries (input) }
    };
}

rustc_queries!()