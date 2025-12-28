macro_rules! deps {
    () => {
        Unsigned!();
        PInt!();
        Z0!();
        Gcd!();
        NonZero!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < U > Gcd < PInt < U > > for Z0 where U : Unsigned + NonZero , { type Output = PInt < U > ; }
    };
}

impl_119!();