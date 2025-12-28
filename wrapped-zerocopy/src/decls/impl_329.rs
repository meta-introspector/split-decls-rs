macro_rules! deps {
    () => {
        Alignment!();
        Aligned!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl Alignment for Aligned { }
    };
}

impl_329!();