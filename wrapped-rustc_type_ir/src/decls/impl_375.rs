macro_rules! deps {
    () => {
        Interner!();
        AliasTerm!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        impl < I : Interner > Eq for AliasTerm < I > { }
    };
}

impl_375!();