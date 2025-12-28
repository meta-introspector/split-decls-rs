macro_rules! deps {
    () => {
        Clause!();
        Interner!();
        Span!();
    };
}

macro_rules! ClauseWithSupertraitSpan {
    () => {
        deps!();
        pub struct ClauseWithSupertraitSpan < I : Interner > { pub clause : I :: Clause , pub supertrait_span : I :: Span , }
    };
}

ClauseWithSupertraitSpan!();