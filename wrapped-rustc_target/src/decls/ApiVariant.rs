macro_rules! ApiVariant {
    () => {
        pub (crate) enum ApiVariant { Default , IoSock , }
    };
}

ApiVariant!()