macro_rules! deps {
    () => {
        TrivialDrop!();
        Iter!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < T > TrivialDrop for slice :: Iter < '_ , T > { }
    };
}

impl_165!();