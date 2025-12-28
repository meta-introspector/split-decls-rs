macro_rules! deps {
    () => {
        Interner!();
        CanonicalVarKind!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < I : Interner > Eq for CanonicalVarKind < I > { }
    };
}

impl_267!();