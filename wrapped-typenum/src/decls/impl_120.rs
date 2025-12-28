macro_rules! deps {
    () => {
        Z0!();
        Unsigned!();
        Gcd!();
        NonZero!();
        PInt!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < U > Gcd < Z0 > for PInt < U > where U : Unsigned + NonZero , { type Output = PInt < U > ; }
    };
}

impl_120!()