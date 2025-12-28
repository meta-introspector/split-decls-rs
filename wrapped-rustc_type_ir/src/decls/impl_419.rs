macro_rules! deps {
    () => {
        WithCachedTypeInfo!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl < T : Eq > Eq for WithCachedTypeInfo < T > { }
    };
}

impl_419!();