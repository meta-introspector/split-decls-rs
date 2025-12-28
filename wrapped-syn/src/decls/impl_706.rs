macro_rules! deps {
    () => {
        IntoSpans!();
    };
}

macro_rules! impl_706 {
    () => {
        deps!();
        impl IntoSpans < Span > for Span { fn into_spans (self) -> Span { self } }
    };
}

impl_706!()