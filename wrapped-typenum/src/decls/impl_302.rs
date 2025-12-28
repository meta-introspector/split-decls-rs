macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        PInt!();
        Abs!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero > Abs for PInt < U > { type Output = Self ; }
    };
}

impl_302!();