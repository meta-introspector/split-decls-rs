macro_rules! deps {
    () => {
        IntoTokenStream!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl IntoTokenStream for Result < TokenStream , Error > { fn into_ts (self) -> TokenStream { match self { Ok (ts) => ts , Err (err) => err . to_compile_error () , } } }
    };
}

impl_7!()