macro_rules! deps {
    () => {
        ClauseKind!();
        Interner!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl < I : Interner > Eq for ClauseKind < I > { }
    };
}

impl_403!();