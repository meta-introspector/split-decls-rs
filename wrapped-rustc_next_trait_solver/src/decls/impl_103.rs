macro_rules! deps {
    () => {
        WipProbeStep!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < I : Interner > Eq for WipProbeStep < I > { }
    };
}

impl_103!()