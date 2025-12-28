macro_rules! deps {
    () => {
        LitStrOrIdent!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl ToTokens for LitStrOrIdent { fn to_tokens (& self , tokens : & mut TokenStream) { match self { LitStrOrIdent :: LitStr (target) => target . to_tokens (tokens) , LitStrOrIdent :: Ident (ident) => ident . to_tokens (tokens) , } } }
    };
}

impl_8!()