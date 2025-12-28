macro_rules! deps {
    () => {
        MockObligationCauseCode!();
        LocalDefId!();
        Span!();
        MockObligationCause!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl MockObligationCause { pub fn new (_span : Span , _body_id : LocalDefId , _code : MockObligationCauseCode) -> Self { MockObligationCause } }
    };
}

impl_43!();