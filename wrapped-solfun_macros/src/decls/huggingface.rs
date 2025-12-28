macro_rules! huggingface {
    () => {
        # [proc_macro] # [decl (fn , name = "huggingface" , vis = "pub" , hash = "039cf2da")] pub fn huggingface (input : TokenStream) -> TokenStream { macros :: huggingface :: huggingface_impl (input) }
    };
}

huggingface!()