macro_rules! deps {
    () => {
        TokensOrDefault!();
    };
}

macro_rules! impl_598 {
    () => {
        deps!();
        impl < 'a , T > ToTokens for TokensOrDefault < 'a , T > where T : ToTokens + Default , { fn to_tokens (& self , tokens : & mut TokenStream) { match self . 0 { Some (t) => t . to_tokens (tokens) , None => T :: default () . to_tokens (tokens) , } } }
    };
}

impl_598!();