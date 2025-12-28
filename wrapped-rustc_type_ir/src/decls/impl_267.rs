macro_rules! deps {
    () => {
        CanonicalVarKind!();
        Interner!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < I : Interner > Eq for CanonicalVarKind < I > { }
    };
}

impl_267!()