macro_rules! deps {
    () => {
        MockObligationCause!();
        TyCtxt!();
        MockParamEnv!();
        MockPredicateObligation!();
        MockPredicate!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl MockPredicateObligation { pub fn new (_tcx : TyCtxt , _cause : MockObligationCause , _param_env : MockParamEnv , _predicate : MockPredicate ,) -> Self { MockPredicateObligation } }
    };
}

impl_45!();