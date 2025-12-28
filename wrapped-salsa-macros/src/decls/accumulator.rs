macro_rules! accumulator {
    () => {
        # [proc_macro_attribute] pub fn accumulator (args : TokenStream , input : TokenStream) -> TokenStream { accumulator :: accumulator (args , input) }
    };
}

accumulator!();