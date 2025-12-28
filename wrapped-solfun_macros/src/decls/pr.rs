macro_rules! pr {
    () => {
        # [proc_macro] # [decl (fn , name = "pr" , vis = "pub" , hash = "439ade7e")] pub fn pr (input : TokenStream) -> TokenStream { macros :: pr :: pr_impl (input) }
    };
}

pr!()