macro_rules! codeberg {
    () => {
        # [proc_macro] # [decl (fn , name = "codeberg" , vis = "pub" , hash = "d565a312")] pub fn codeberg (input : TokenStream) -> TokenStream { macros :: codeberg :: codeberg_impl (input) }
    };
}

codeberg!()