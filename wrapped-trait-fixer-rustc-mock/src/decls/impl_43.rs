macro_rules! deps {
    () => {
        MockObligationCause!();
        MockObligationCauseCode!();
        Span!();
        LocalDefId!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl MockObligationCause { pub fn new (_span : Span , _body_id : LocalDefId , _code : MockObligationCauseCode) -> Self { MockObligationCause } }
    };
}

impl_43!()