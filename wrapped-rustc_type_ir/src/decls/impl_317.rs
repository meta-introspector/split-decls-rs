macro_rules! deps {
    () => {
        Interner!();
        GenericArgKind!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < I : Interner > Eq for GenericArgKind < I > { }
    };
}

impl_317!()