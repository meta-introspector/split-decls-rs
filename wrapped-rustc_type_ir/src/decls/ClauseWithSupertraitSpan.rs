macro_rules! deps {
    () => {
        Span!();
        Clause!();
        Interner!();
    };
}

macro_rules! ClauseWithSupertraitSpan {
    () => {
        deps!();
        pub struct ClauseWithSupertraitSpan < I : Interner > { pub clause : I :: Clause , pub supertrait_span : I :: Span , }
    };
}

ClauseWithSupertraitSpan!()