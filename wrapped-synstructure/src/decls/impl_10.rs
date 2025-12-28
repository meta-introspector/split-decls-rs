macro_rules! deps {
    () => {
        BindingInfo!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl ToTokens for BindingInfo < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { self . binding . to_tokens (tokens) ; } }
    };
}

impl_10!()