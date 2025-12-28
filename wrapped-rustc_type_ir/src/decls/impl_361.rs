macro_rules! deps {
    () => {
        ExistentialPredicate!();
        Interner!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl < I : Interner > Eq for ExistentialPredicate < I > { }
    };
}

impl_361!()