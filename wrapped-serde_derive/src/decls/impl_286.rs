macro_rules! impl_286 {
    () => {
        impl ToTokens for private { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { tokens . append (self . ident ()) ; } }
    };
}

impl_286!()