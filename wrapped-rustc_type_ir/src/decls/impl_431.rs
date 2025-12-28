macro_rules! deps {
    () => {
        TyKind!();
        Interner!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl < I : Interner > Eq for TyKind < I > { }
    };
}

impl_431!();