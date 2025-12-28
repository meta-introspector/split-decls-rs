macro_rules! deps {
    () => {
        ExistentialProjection!();
        Interner!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        impl < I : Interner > Eq for ExistentialProjection < I > { }
    };
}

impl_368!();