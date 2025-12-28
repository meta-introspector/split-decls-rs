macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! attr {
    () => {
        deps!();
        # [proc_macro_attribute] pub fn attr (args : TokenStream , input : TokenStream) -> TokenStream { attr :: parse (args) . and_then (| args | expand :: try_attr (args , input)) . unwrap_or_else (Error :: into_compile_error) }
    };
}

attr!();