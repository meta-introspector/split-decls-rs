macro_rules! IntoTokenStream {
    () => {
        trait IntoTokenStream { fn into_ts (self) -> TokenStream ; }
    };
}

IntoTokenStream!()