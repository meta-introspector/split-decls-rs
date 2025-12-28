macro_rules! deps {
    () => {
        TrivialDrop!();
        IntoIter!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < T > TrivialDrop for option :: IntoIter < & mut T > { }
    };
}

impl_168!();