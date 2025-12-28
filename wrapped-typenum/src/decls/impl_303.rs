macro_rules! deps {
    () => {
        Unsigned!();
        NonZero!();
        Abs!();
        NInt!();
        PInt!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero > Abs for NInt < U > { type Output = PInt < U > ; }
    };
}

impl_303!();