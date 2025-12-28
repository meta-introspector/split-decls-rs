macro_rules! deps {
    () => {
        IntoSpans!();
    };
}

macro_rules! impl_709 {
    () => {
        deps!();
        impl IntoSpans < [Span ; 3] > for Span { fn into_spans (self) -> [Span ; 3] { [self , self , self] } }
    };
}

impl_709!();