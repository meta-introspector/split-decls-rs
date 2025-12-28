macro_rules! deps {
    () => {
        FnHeader!();
        Interner!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < I : Interner > Eq for FnHeader < I > { }
    };
}

impl_473!();