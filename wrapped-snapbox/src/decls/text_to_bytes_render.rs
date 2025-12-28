macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! text_to_bytes_render {
    () => {
        deps!();
        # [test] fn text_to_bytes_render () { let d = Data :: text (String :: from ("test")) ; let bytes = d . to_bytes () . unwrap () ; let bytes = String :: from_utf8 (bytes) . unwrap () ; let rendered = d . render () . unwrap () ; assert_eq ! (bytes , rendered) ; }
    };
}

text_to_bytes_render!();