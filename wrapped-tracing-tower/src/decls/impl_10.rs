macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < S > Service < S > { pub fn new (inner : S , span : tracing :: Span) -> Self { Self { inner , span } } }
    };
}

impl_10!()