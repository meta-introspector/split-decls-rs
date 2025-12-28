macro_rules! deps {
    () => {
        Body!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl ToTokens for Body < '_ > { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { self . brace_token . surround (tokens , | tokens | { for stmt in self . stmts { stmt . to_tokens (tokens) ; } }) ; } }
    };
}

impl_24!()