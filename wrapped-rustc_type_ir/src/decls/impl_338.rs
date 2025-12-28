macro_rules! deps {
    () => {
        Interner!();
        PatternKind!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl < I : Interner > Eq for PatternKind < I > { }
    };
}

impl_338!()