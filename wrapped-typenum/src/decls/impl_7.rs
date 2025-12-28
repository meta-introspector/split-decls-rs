macro_rules! deps {
    () => {
        B1!();
        NonZero!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl NonZero for B1 { }
    };
}

impl_7!();