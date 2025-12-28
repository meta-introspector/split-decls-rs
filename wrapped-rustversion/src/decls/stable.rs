macro_rules! stable {
    () => {
        # [proc_macro_attribute] pub fn stable (args : TokenStream , input : TokenStream) -> TokenStream { expand :: cfg ("stable" , args , input) }
    };
}

stable!();