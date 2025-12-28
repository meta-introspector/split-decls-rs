macro_rules! oil {
    () => {
        # [proc_macro] # [decl (fn , name = "oil" , vis = "pub" , hash = "a1e2d08a")] pub fn oil (input : TokenStream) -> TokenStream { macros :: oil :: oil_impl (input) }
    };
}

oil!()