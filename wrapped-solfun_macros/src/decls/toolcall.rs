macro_rules! toolcall {
    () => {
        # [proc_macro] # [decl (fn , name = "toolcall" , vis = "pub" , hash = "e3d5663a")] pub fn toolcall (input : TokenStream) -> TokenStream { macros :: toolcall :: toolcall_impl (input) }
    };
}

toolcall!()