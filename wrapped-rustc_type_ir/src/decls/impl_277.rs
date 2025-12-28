macro_rules! deps {
    () => {
        ConstKind!();
        Interner!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < I : Interner > Eq for ConstKind < I > { }
    };
}

impl_277!();