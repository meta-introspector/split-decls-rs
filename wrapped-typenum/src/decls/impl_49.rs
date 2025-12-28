macro_rules! deps {
    () => {
        Unsigned!();
        NonZero!();
        PInt!();
        PowerOfTwo!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero + PowerOfTwo > PowerOfTwo for PInt < U > { }
    };
}

impl_49!()