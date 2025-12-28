macro_rules! deps {
    () => {
        FnSigTys!();
        Interner!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl < I : Interner > Eq for FnSigTys < I > { }
    };
}

impl_469!()