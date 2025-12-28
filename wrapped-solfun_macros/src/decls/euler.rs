macro_rules! euler {
    () => {
        # [proc_macro] # [decl (fn , name = "euler" , vis = "pub" , hash = "e63b3b8a")] pub fn euler (input : TokenStream) -> TokenStream { macros :: euler :: euler_impl (input) }
    };
}

euler!();