macro_rules! deps {
    () => {
        Interner!();
        CanonicalVarValues!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < I : Interner > Eq for CanonicalVarValues < I > { }
    };
}

impl_270!()