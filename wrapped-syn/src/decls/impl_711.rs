macro_rules! deps {
    () => {
        IntoSpans!();
    };
}

macro_rules! impl_711 {
    () => {
        deps!();
        impl IntoSpans < [Span ; 2] > for [Span ; 2] { fn into_spans (self) -> [Span ; 2] { self } }
    };
}

impl_711!()