macro_rules! reddit {
    () => {
        # [proc_macro] # [decl (fn , name = "reddit" , vis = "pub" , hash = "ae6f96ee")] pub fn reddit (input : TokenStream) -> TokenStream { macros :: reddit :: reddit_impl (input) }
    };
}

reddit!();