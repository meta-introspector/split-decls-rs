macro_rules! deps {
    () => {
        Interner!();
        ExistentialTraitRef!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl < I : Interner > Eq for ExistentialTraitRef < I > { }
    };
}

impl_364!()