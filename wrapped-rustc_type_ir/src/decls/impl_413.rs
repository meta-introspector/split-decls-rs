macro_rules! deps {
    () => {
        RegionKind!();
        Interner!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl < I : Interner > Eq for RegionKind < I > { }
    };
}

impl_413!()