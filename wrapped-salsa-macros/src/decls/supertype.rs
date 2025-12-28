macro_rules! supertype {
    () => {
        # [proc_macro_derive (Supertype)] pub fn supertype (input : TokenStream) -> TokenStream { supertype :: supertype (input) }
    };
}

supertype!()