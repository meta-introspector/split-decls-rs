macro_rules! deps {
    () => {
        Unsigned!();
        Gcd!();
        Gcf!();
        NonZero!();
        PInt!();
        NInt!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < U1 , U2 > Gcd < PInt < U2 > > for NInt < U1 > where U1 : Unsigned + NonZero + Gcd < U2 > , U2 : Unsigned + NonZero , Gcf < U1 , U2 > : Unsigned + NonZero , { type Output = PInt < Gcf < U1 , U2 > > ; }
    };
}

impl_124!();