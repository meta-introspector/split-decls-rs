macro_rules! deps {
    () => {
        IntoSpans!();
    };
}

macro_rules! impl_710 {
    () => {
        deps!();
        impl IntoSpans < [Span ; 1] > for [Span ; 1] { fn into_spans (self) -> [Span ; 1] { self } }
    };
}

impl_710!()