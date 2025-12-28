macro_rules! deps {
    () => {
        Interner!();
        CoroutineWitnessTypes!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl < I : Interner > Eq for CoroutineWitnessTypes < I > { }
    };
}

impl_475!();