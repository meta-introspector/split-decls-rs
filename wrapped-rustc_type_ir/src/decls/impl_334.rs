macro_rules! deps {
    () => {
        OpaqueTypeKey!();
        Interner!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl < I : Interner > Eq for OpaqueTypeKey < I > { }
    };
}

impl_334!()