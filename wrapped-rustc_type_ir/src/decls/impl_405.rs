macro_rules! deps {
    () => {
        PredicateKind!();
        Interner!();
    };
}

macro_rules! impl_405 {
    () => {
        deps!();
        impl < I : Interner > Eq for PredicateKind < I > { }
    };
}

impl_405!()