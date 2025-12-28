macro_rules! deps {
    () => {
        Gcd!();
        NInt!();
        Z0!();
        Unsigned!();
        NonZero!();
        PInt!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < U > Gcd < NInt < U > > for Z0 where U : Unsigned + NonZero , { type Output = PInt < U > ; }
    };
}

impl_121!();