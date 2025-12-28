macro_rules! deps {
    () => {
        DeString!();
    };
}

macro_rules! get_key_span {
    () => {
        deps!();
        fn get_key_span (key : & Spanned < DeString < '_ > >) -> toml_parser :: Span { let key_span = key . span () ; toml_parser :: Span :: new_unchecked (key_span . start , key_span . end) }
    };
}

get_key_span!();