macro_rules! deps {
    () => {
        TraitPredicate!();
        Interner!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl < I : Interner > Eq for TraitPredicate < I > { }
    };
}

impl_348!()