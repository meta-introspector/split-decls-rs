macro_rules! all {
    () => {
        # [proc_macro_attribute] pub fn all (args : TokenStream , input : TokenStream) -> TokenStream { expand :: cfg ("all" , args , input) }
    };
}

all!()