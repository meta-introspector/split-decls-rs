macro_rules! nightly {
    () => {
        # [proc_macro_attribute] pub fn nightly (args : TokenStream , input : TokenStream) -> TokenStream { expand :: cfg ("nightly" , args , input) }
    };
}

nightly!();