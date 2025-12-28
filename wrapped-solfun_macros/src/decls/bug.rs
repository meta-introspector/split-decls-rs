macro_rules! bug {
    () => {
        # [proc_macro] # [decl (fn , name = "bug" , vis = "pub" , hash = "363cb3e8")] pub fn bug (input : TokenStream) -> TokenStream { macros :: bug :: bug_impl (input) }
    };
}

bug!();