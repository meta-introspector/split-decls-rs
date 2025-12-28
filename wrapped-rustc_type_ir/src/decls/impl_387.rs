macro_rules! deps {
    () => {
        NormalizesTo!();
        Interner!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl < I : Interner > Eq for NormalizesTo < I > { }
    };
}

impl_387!()