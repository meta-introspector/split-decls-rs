macro_rules! codegen {
    () => {
        # [proc_macro] # [decl (fn , name = "codegen" , vis = "pub" , hash = "7a67ad6e")] pub fn codegen (input : TokenStream) -> TokenStream { macros :: codegen :: codegen_impl (input) }
    };
}

codegen!();