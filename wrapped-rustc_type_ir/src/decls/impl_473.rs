macro_rules! deps {
    () => {
        Interner!();
        FnHeader!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < I : Interner > Eq for FnHeader < I > { }
    };
}

impl_473!()