macro_rules! deps {
    () => {
        Fields!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl ToTokens for Fields { fn to_tokens (& self , tokens : & mut TokenStream) { self . 0 . to_tokens (tokens) } }
    };
}

impl_21!()