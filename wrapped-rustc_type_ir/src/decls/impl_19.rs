macro_rules! deps {
    () => {
        Clause!();
        Span!();
        Interner!();
        ClauseWithSupertraitSpan!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < I : Interner > ClauseWithSupertraitSpan < I > { pub fn new (clause : I :: Clause , span : I :: Span) -> Self { ClauseWithSupertraitSpan { clause , supertrait_span : span } } }
    };
}

impl_19!()