macro_rules! deps {
    () => {
        GetSpan!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T > GetSpan < T > for tracing :: Span { # [inline] fn span_for (& self , _ : & T) -> tracing :: Span { self . clone () } }
    };
}

impl_10!()