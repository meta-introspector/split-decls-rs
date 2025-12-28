macro_rules! deps {
    () => {
        Interner!();
        TermKind!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < I : Interner > Eq for TermKind < I > { }
    };
}

impl_319!()