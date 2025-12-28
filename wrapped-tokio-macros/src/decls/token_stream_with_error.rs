macro_rules! token_stream_with_error {
    () => {
        fn token_stream_with_error (mut tokens : TokenStream , error : syn :: Error) -> TokenStream { tokens . extend (error . into_compile_error ()) ; tokens }
    };
}

token_stream_with_error!();