macro_rules! deps {
    () => {
        Unsigned!();
        Abs!();
        PInt!();
        NonZero!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero > Abs for PInt < U > { type Output = Self ; }
    };
}

impl_302!()