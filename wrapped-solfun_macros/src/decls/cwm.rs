macro_rules! cwm {
    () => {
        # [proc_macro] # [decl (fn , name = "cwm" , vis = "pub" , hash = "b40e68a5")] pub fn cwm (input : TokenStream) -> TokenStream { macros :: cwm :: cwm_impl (input) }
    };
}

cwm!()