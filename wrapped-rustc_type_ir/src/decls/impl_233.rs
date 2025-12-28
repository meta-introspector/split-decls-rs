macro_rules! deps {
    () => {
        EarlyBinder!();
        Interner!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < I : Interner , T : Eq > Eq for EarlyBinder < I , T > { }
    };
}

impl_233!();