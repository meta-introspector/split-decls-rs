macro_rules! deps {
    () => {
        Interner!();
        HostEffectPredicate!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        impl < I : Interner > Eq for HostEffectPredicate < I > { }
    };
}

impl_391!();