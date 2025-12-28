macro_rules! deps {
    () => {
        Interner!();
        AliasTy!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl < I : Interner > Eq for AliasTy < I > { }
    };
}

impl_435!();