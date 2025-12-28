macro_rules! deps {
    () => {
        Gcf!();
        Gcd!();
        Unsigned!();
        PInt!();
        NonZero!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < U1 , U2 > Gcd < PInt < U2 > > for PInt < U1 > where U1 : Unsigned + NonZero + Gcd < U2 > , U2 : Unsigned + NonZero , Gcf < U1 , U2 > : Unsigned + NonZero , { type Output = PInt < Gcf < U1 , U2 > > ; }
    };
}

impl_123!();