macro_rules! deps {
    () => {
        IntoSpans!();
    };
}

macro_rules! impl_714 {
    () => {
        deps!();
        impl IntoSpans < DelimSpan > for DelimSpan { fn into_spans (self) -> DelimSpan { self } }
    };
}

impl_714!()