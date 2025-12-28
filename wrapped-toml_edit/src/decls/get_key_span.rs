macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! get_key_span {
    () => {
        deps!();
        fn get_key_span (key : & Key) -> Option < toml_parser :: Span > { key . as_repr () . and_then (| r | r . span ()) . map (| s | toml_parser :: Span :: new_unchecked (s . start , s . end)) }
    };
}

get_key_span!();