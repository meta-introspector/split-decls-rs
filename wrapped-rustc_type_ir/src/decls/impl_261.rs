macro_rules! deps {
    () => {
        CanonicalQueryInput!();
        Interner!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl < I : Interner , V : Eq > Eq for CanonicalQueryInput < I , V > { }
    };
}

impl_261!();