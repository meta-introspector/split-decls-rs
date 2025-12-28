macro_rules! deps {
    () => {
        IntoSpans!();
    };
}

macro_rules! impl_712 {
    () => {
        deps!();
        impl IntoSpans < [Span ; 3] > for [Span ; 3] { fn into_spans (self) -> [Span ; 3] { self } }
    };
}

impl_712!();