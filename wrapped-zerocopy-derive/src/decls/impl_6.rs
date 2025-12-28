macro_rules! deps {
    () => {
        IntoTokenStream!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl IntoTokenStream for TokenStream { fn into_ts (self) -> TokenStream { self } }
    };
}

impl_6!()