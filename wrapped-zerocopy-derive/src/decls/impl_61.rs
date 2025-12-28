macro_rules! deps {
    () => {
        IntoTokenStream!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl IntoTokenStream for TokenStream { fn into_ts (self) -> TokenStream { self } }
    };
}

impl_61!();