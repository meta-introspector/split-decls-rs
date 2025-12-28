macro_rules! deps {
    () => {
        ProjectionPredicate!();
        Interner!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl < I : Interner > Eq for ProjectionPredicate < I > { }
    };
}

impl_382!()