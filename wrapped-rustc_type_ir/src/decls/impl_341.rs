macro_rules! deps {
    () => {
        Interner!();
        OutlivesPredicate!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < I : Interner , A : Eq > Eq for OutlivesPredicate < I , A > { }
    };
}

impl_341!()