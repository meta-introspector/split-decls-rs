macro_rules! since {
    () => {
        # [proc_macro_attribute] pub fn since (args : TokenStream , input : TokenStream) -> TokenStream { expand :: cfg ("since" , args , input) }
    };
}

since!()