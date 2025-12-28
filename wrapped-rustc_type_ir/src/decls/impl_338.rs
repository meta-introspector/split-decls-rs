macro_rules! deps {
    () => {
        PatternKind!();
        Interner!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl < I : Interner > Eq for PatternKind < I > { }
    };
}

impl_338!();