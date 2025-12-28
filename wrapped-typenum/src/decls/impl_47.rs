macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        NInt!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero > NonZero for NInt < U > { }
    };
}

impl_47!();