macro_rules! PunctExt {
    () => {
        pub (crate) trait PunctExt { fn new_spanned (ch : char , spacing : Spacing , span : Span) -> Self ; }
    };
}

PunctExt!();