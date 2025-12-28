macro_rules! deps {
    () => {
        NonZero!();
        B1!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl NonZero for B1 { }
    };
}

impl_7!()