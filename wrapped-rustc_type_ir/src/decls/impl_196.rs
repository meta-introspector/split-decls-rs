macro_rules! deps {
    () => {
        NestedNormalizationGoals!();
        Interner!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl < I : Interner > Eq for NestedNormalizationGoals < I > { }
    };
}

impl_196!()