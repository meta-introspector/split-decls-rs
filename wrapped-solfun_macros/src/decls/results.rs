macro_rules! results {
    () => {
        # [proc_macro] # [decl (fn , name = "results" , vis = "pub" , hash = "5f3457b2")] pub fn results (input : TokenStream) -> TokenStream { macros :: results :: results_impl (input) }
    };
}

results!();