macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl ToTokens for Name { fn to_tokens (& self , tokens : & mut TokenStream) { LitStr :: new (& self . value , self . span) . to_tokens (tokens) ; } }
    };
}

impl_66!();