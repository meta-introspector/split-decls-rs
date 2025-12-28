macro_rules! deps {
    () => {
        PredefinedOpaquesData!();
        Interner!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < I : Interner > Eq for PredefinedOpaquesData < I > { }
    };
}

impl_185!()