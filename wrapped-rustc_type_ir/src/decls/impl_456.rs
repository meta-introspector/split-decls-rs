macro_rules! deps {
    () => {
        FnSig!();
        Interner!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl < I : Interner > Eq for FnSig < I > { }
    };
}

impl_456!()