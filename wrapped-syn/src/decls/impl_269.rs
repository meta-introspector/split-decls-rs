macro_rules! deps {
    () => {
        PunctExt!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl PunctExt for Punct { fn new_spanned (ch : char , spacing : Spacing , span : Span) -> Self { let mut punct = Punct :: new (ch , spacing) ; punct . set_span (span) ; punct } }
    };
}

impl_269!();