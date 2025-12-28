macro_rules! deps {
    () => {
        Interner!();
        OpaqueTypeKey!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl < I : Interner > Eq for OpaqueTypeKey < I > { }
    };
}

impl_334!();