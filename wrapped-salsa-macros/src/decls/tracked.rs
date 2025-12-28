macro_rules! tracked {
    () => {
        # [proc_macro_attribute] pub fn tracked (args : TokenStream , input : TokenStream) -> TokenStream { tracked :: tracked (args , input) }
    };
}

tracked!();