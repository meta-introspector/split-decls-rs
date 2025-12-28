macro_rules! deps {
    () => {
        MockParamEnv!();
        MockObligationCause!();
        MockPredicate!();
        MockPredicateObligation!();
        TyCtxt!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl MockPredicateObligation { pub fn new (_tcx : TyCtxt , _cause : MockObligationCause , _param_env : MockParamEnv , _predicate : MockPredicate ,) -> Self { MockPredicateObligation } }
    };
}

impl_45!()