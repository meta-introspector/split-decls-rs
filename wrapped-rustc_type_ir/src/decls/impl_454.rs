macro_rules! deps {
    () => {
        TypeAndMut!();
        Interner!();
    };
}

macro_rules! impl_454 {
    () => {
        deps!();
        impl < I : Interner > Eq for TypeAndMut < I > { }
    };
}

impl_454!();