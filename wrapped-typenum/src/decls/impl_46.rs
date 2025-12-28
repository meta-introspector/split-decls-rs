macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        PInt!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero > NonZero for PInt < U > { }
    };
}

impl_46!();