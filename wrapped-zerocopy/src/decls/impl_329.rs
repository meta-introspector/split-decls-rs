macro_rules! deps {
    () => {
        Aligned!();
        Alignment!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl Alignment for Aligned { }
    };
}

impl_329!()