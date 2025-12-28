macro_rules! deps {
    () => {
        Interner!();
        ClauseKind!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl < I : Interner > Eq for ClauseKind < I > { }
    };
}

impl_403!()