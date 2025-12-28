macro_rules! biosemiotic {
    () => {
        # [proc_macro] # [decl (fn , name = "biosemiotic" , vis = "pub" , hash = "81f81afc")] pub fn biosemiotic (input : TokenStream) -> TokenStream { macros :: biosemiotic :: biosemiotic_impl (input) }
    };
}

biosemiotic!()