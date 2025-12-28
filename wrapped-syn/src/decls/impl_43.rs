macro_rules! impl_43 {
    () => {
        # [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Underscore { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Ident :: new ("_" , self . span)) ; } }
    };
}

impl_43!();