macro_rules! deps {
    () => {
        VarianceDiagInfo!();
        Interner!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < I : Interner > Eq for VarianceDiagInfo < I > { }
    };
}

impl_111!()