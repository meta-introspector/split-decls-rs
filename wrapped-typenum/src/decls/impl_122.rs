macro_rules! deps {
    () => {
        Z0!();
        NInt!();
        Unsigned!();
        PInt!();
        Gcd!();
        NonZero!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < U > Gcd < Z0 > for NInt < U > where U : Unsigned + NonZero , { type Output = PInt < U > ; }
    };
}

impl_122!();