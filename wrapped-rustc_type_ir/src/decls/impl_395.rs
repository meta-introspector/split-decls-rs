macro_rules! deps {
    () => {
        SubtypePredicate!();
        Interner!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        impl < I : Interner > Eq for SubtypePredicate < I > { }
    };
}

impl_395!();