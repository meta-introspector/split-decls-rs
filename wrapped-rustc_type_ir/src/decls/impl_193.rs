macro_rules! deps {
    () => {
        ExternalConstraintsData!();
        Interner!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < I : Interner > Eq for ExternalConstraintsData < I > { }
    };
}

impl_193!();