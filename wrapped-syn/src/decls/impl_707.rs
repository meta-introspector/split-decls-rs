macro_rules! deps {
    () => {
        IntoSpans!();
    };
}

macro_rules! impl_707 {
    () => {
        deps!();
        impl IntoSpans < [Span ; 1] > for Span { fn into_spans (self) -> [Span ; 1] { [self] } }
    };
}

impl_707!()