macro_rules! deps {
    () => {
        Iter!();
        TrivialDrop!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < T > TrivialDrop for slice :: Iter < '_ , T > { }
    };
}

impl_165!()