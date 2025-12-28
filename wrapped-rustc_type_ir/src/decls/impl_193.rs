macro_rules! deps {
    () => {
        Interner!();
        ExternalConstraintsData!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < I : Interner > Eq for ExternalConstraintsData < I > { }
    };
}

impl_193!()