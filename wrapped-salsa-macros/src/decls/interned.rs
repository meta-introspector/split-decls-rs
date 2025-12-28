macro_rules! interned {
    () => {
        # [proc_macro_attribute] pub fn interned (args : TokenStream , input : TokenStream) -> TokenStream { interned :: interned (args , input) }
    };
}

interned!()