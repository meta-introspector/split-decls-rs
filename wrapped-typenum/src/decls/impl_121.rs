macro_rules! deps {
    () => {
        Gcd!();
        Unsigned!();
        Z0!();
        NonZero!();
        NInt!();
        PInt!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < U > Gcd < NInt < U > > for Z0 where U : Unsigned + NonZero , { type Output = PInt < U > ; }
    };
}

impl_121!()