macro_rules! deps {
    () => {
        Nothing!();
    };
}

macro_rules! impl_532 {
    () => {
        deps!();
        # [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] impl ToTokens for Nothing { fn to_tokens (& self , tokens : & mut TokenStream) { let _ = tokens ; } }
    };
}

impl_532!();