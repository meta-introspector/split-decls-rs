macro_rules! deps {
    () => {
        Span!();
        EnteredSpan!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Deref for EnteredSpan { type Target = Span ; # [inline] fn deref (& self) -> & Span { & self . span } }
    };
}

impl_77!();