macro_rules! deps {
    () => {
        Interner!();
        TyKind!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl < I : Interner > Eq for TyKind < I > { }
    };
}

impl_431!()