macro_rules! deps {
    () => {
        CoercePredicate!();
        Interner!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl < I : Interner > Eq for CoercePredicate < I > { }
    };
}

impl_397!();