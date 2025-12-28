macro_rules! videogen {
    () => {
        # [proc_macro] # [decl (fn , name = "videogen" , vis = "pub" , hash = "6e0880ff")] pub fn videogen (input : TokenStream) -> TokenStream { macros :: videogen :: videogen_impl (input) }
    };
}

videogen!()