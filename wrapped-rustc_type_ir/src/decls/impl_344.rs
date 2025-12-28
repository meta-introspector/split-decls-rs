macro_rules! deps {
    () => {
        Interner!();
        TraitRef!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl < I : Interner > Eq for TraitRef < I > { }
    };
}

impl_344!()