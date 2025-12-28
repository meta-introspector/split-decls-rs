macro_rules! deps {
    () => {
        WipProbe!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < I : Interner > Eq for WipProbe < I > { }
    };
}

impl_100!()