macro_rules! deps {
    () => {
        PInt!();
        Z0!();
        Gcd!();
        Unsigned!();
        NonZero!();
        NInt!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < U > Gcd < Z0 > for NInt < U > where U : Unsigned + NonZero , { type Output = PInt < U > ; }
    };
}

impl_122!()