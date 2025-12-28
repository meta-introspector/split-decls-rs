macro_rules! deps {
    () => {
        IntoSpans!();
        TokenMarker!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl < S > IntoSpans < S > for TokenMarker { fn into_spans (self) -> S { match self { } } }
    };
}

impl_467!()