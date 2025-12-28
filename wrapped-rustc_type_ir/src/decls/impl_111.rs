macro_rules! deps {
    () => {
        Interner!();
        VarianceDiagInfo!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < I : Interner > Eq for VarianceDiagInfo < I > { }
    };
}

impl_111!();