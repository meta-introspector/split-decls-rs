macro_rules! deps {
    () => {
        GetSpan!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T , F > GetSpan < T > for F where F : Fn (& T) -> tracing :: Span , { # [inline] fn span_for (& self , target : & T) -> tracing :: Span { (self) (target) } }
    };
}

impl_8!()