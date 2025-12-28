macro_rules! deps {
    () => {
        IntoSpans!();
    };
}

macro_rules! impl_708 {
    () => {
        deps!();
        impl IntoSpans < [Span ; 2] > for Span { fn into_spans (self) -> [Span ; 2] { [self , self] } }
    };
}

impl_708!()