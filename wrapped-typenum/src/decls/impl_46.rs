macro_rules! deps {
    () => {
        Unsigned!();
        NonZero!();
        PInt!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero > NonZero for PInt < U > { }
    };
}

impl_46!()