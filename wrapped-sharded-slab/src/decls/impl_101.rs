macro_rules! deps {
    () => {
        Lifecycle!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < C > Eq for Lifecycle < C > { }
    };
}

impl_101!();