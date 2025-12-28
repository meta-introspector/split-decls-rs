macro_rules! deps {
    () => {
        CanonicalVarValues!();
        Interner!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < I : Interner > Eq for CanonicalVarValues < I > { }
    };
}

impl_270!();