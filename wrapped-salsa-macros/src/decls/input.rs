macro_rules! input {
    () => {
        # [proc_macro_attribute] pub fn input (args : TokenStream , input : TokenStream) -> TokenStream { input :: input (args , input) }
    };
}

input!()