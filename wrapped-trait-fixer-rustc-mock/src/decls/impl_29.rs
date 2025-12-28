macro_rules! deps {
    () => {
        MockPredicate!();
        MockInferCtxtAt!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl MockInferCtxtAt { pub fn predicate_may_hold (self , _predicate : & MockPredicate) -> bool { true } }
    };
}

impl_29!()