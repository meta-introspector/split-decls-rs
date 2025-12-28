macro_rules! not {
    () => {
        # [proc_macro_attribute] pub fn not (args : TokenStream , input : TokenStream) -> TokenStream { expand :: cfg ("not" , args , input) }
    };
}

not!()