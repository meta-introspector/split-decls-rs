macro_rules! deps {
    () => {
        FieldKind!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl ToTokens for FieldKind { fn to_tokens (& self , tokens : & mut TokenStream) { match self { FieldKind :: Debug => tokens . extend (quote ! { ? }) , FieldKind :: Display => tokens . extend (quote ! { % }) , _ => { } } } }
    };
}

impl_24!()